import Navbar from "../components/Navbar";
import { useEffect, useState } from "react";
import { Link } from "react-router-dom";
import { apiUrl } from "../config";
import { ArrowLeftIcon, CheckmarkIcon, XMarkIcon } from "@navikt/aksel-icons";

type ContributionStatus = "Proposed" | "Approved" | "Denied";

type ContributionKind =
  | { Text: Record<string, unknown> }
  | { File: { AddResource: { title: string; key: string } } }
  | { File: { RemoveResource: { resource_id: string } } };

type Contribution = {
  aggregate_id: string;
  course_id: string;
  contribution: ContributionKind;
  status: ContributionStatus;
};

function isFileAddResource(
  kind: ContributionKind
): kind is { File: { AddResource: { title: string; key: string } } } {
  return "File" in kind && "AddResource" in kind.File;
}

function describeContribution(kind: ContributionKind): string {
  if ("File" in kind) {
    if ("AddResource" in kind.File) return `Add resource: "${kind.File.AddResource.title}"`;
    if ("RemoveResource" in kind.File) return `Remove resource ${kind.File.RemoveResource.resource_id}`;
  }
  if ("Text" in kind) {
    const [action, payload] = Object.entries(kind.Text)[0] ?? [];
    return `${action ?? "Unknown"}: ${JSON.stringify(payload ?? {})}`;
  }
  return "Unknown contribution";
}

export default function AdminContributions() {
  const [contributions, setContributions] = useState<Contribution[] | null>(null);
  const [busyId, setBusyId] = useState<string | null>(null);

  const fetchContributions = () => {
    fetch(apiUrl("contributions"), { credentials: "include" })
      .then((r) => {
        if (!r.ok) throw new Error(`Failed to load contributions (${r.status})`);
        return r.json() as Promise<Contribution[]>;
      })
      .then(setContributions)
      .catch(() => setContributions([]));
  };

  useEffect(() => {
    fetchContributions();
  }, []);

  const moderate = async (contributionId: string, verdict: "Approve" | "Deny") => {
    setBusyId(contributionId);
    try {
      const res = await fetch(apiUrl("contributions"), {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        credentials: "include",
        body: JSON.stringify({
          Moderate: { contribution_id: contributionId, verdict },
        }),
      });
      if (!res.ok) throw new Error("moderation failed");
      fetchContributions();
    } catch {
      alert(`Could not ${verdict.toLowerCase()} this contribution.`);
    } finally {
      setBusyId(null);
    }
  };

  const pending = contributions?.filter((c) => c.status === "Proposed") ?? [];

  return (
    <div className="min-h-screen bg-surface-dark text-text">
      <Navbar />
      <main className="container mx-auto max-w-4xl p-6">
        <Link
          to="/admin"
          className="inline-flex items-center gap-1.5 text-text-secondary hover:text-text transition-colors mb-4"
        >
          <ArrowLeftIcon aria-hidden /> Back
        </Link>

        <h1 className="text-2xl font-bold text-primary mb-6">Pending contributions</h1>

        {contributions === null ? (
          <div className="text-text-muted">Loading...</div>
        ) : pending.length === 0 ? (
          <div className="text-text-muted">Nothing waiting for review.</div>
        ) : (
          <div className="flex flex-col gap-3">
            {pending.map((c) => (
              <div
                key={c.aggregate_id}
                className="bg-bg border border-surface rounded-lg p-4 flex items-start justify-between gap-4"
              >
                <div>
                  <p className="text-text font-medium">{describeContribution(c.contribution)}</p>
                  <p className="text-text-secondary text-sm mt-1">Course: {c.course_id}</p>
                  {isFileAddResource(c.contribution) && (
                    <a
                      href={apiUrl(
                        `files/${c.contribution.File.AddResource.key}/preview?course_id=${c.course_id}`
                      )}
                      target="_blank"
                      rel="noreferrer"
                      className="text-accent hover:text-accent-dark text-sm underline mt-1 inline-block"
                    >
                      Preview file
                    </a>
                  )}
                </div>
                <div className="flex gap-2 shrink-0">
                  <button
                    onClick={() => moderate(c.aggregate_id, "Approve")}
                    disabled={busyId === c.aggregate_id}
                    className="inline-flex items-center gap-1.5 bg-accent hover:bg-accent-dark text-white px-3 py-1.5 rounded text-sm transition disabled:opacity-60"
                  >
                    <CheckmarkIcon aria-hidden /> Approve
                  </button>
                  <button
                    onClick={() => moderate(c.aggregate_id, "Deny")}
                    disabled={busyId === c.aggregate_id}
                    className="inline-flex items-center gap-1.5 bg-red-600 hover:bg-red-700 text-white px-3 py-1.5 rounded text-sm transition disabled:opacity-60"
                  >
                    <XMarkIcon aria-hidden /> Deny
                  </button>
                </div>
              </div>
            ))}
          </div>
        )}
      </main>
    </div>
  );
}
