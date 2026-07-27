import Navbar from "../components/Navbar";
import { useEffect, useState } from "react";
import { Link } from "react-router-dom";
import { apiUrl } from "../config";
import {
  CheckmarkIcon,
  XMarkIcon,
  TrashIcon,
  ExclamationmarkTriangleFillIcon,
  PersonIcon,
  ShieldIcon
} from "@navikt/aksel-icons";
import {
  mockReportedContent,
  reportCategoryLabels,
} from "../data/mockData";

type Contribution = {
  id: string;
  title?: string;
  comment?: string;
  authorName?: string;
  courseName?: string;
  [key: string]: unknown;
};

export default function Profile() {
  const [contributions, setContributions] = useState<Contribution[] | null>(null);
  const [actionLoading, setActionLoading] = useState<string | null>(null);

  useEffect(() => {
    fetchContributions();
  }, []);

  const fetchContributions = () => {
    fetch(apiUrl("contributions"), { credentials: "include" })
      .then((r) => {
        if (!r.ok) throw new Error(`Failed to load contributions (${r.status})`);
        return r.json() as Promise<Contribution[]>;
      })
      .then((data) => setContributions(data))
      .catch(() => setContributions([]));
  };

  const handleModerate = async (contributionId: string, verdict: "Approve" | "Reject") => {
    setActionLoading(contributionId);
    try {
      const res = await fetch(apiUrl("contributions"), {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        credentials: "include",
        body: JSON.stringify({
          Moderate: {
            contribution_id: contributionId,
            verdict: verdict,
          },
        }),
      });

      if (!res.ok) throw new Error("Moderation failed");

      setContributions((prev) => (prev ? prev.filter((c) => c.id !== contributionId) : []));
    } catch {
      alert("Failed to process contribution");
    } finally {
      setActionLoading(null);
    }
  };

  return (
    <div className="min-h-screen bg-surface-dark text-text">
      <Navbar />
      <main className="container mx-auto max-w-4xl p-6">
        <div className="flex items-center justify-between gap-4 mb-10">
          <div className="inline-flex items-center justify-center w-10 h-10 rounded-full shrink-0 bg-surface-light text-text border border-border no-underline transition-colors hover:bg-surface">
            <PersonIcon aria-hidden fontSize="1.375rem" />
          </div>

          <Link
            to="/staff"
            className="inline-flex items-center gap-1.5 bg-accent hover:bg-accent-dark text-white px-4 py-2 rounded transition no-underline"
          >
            <ShieldIcon aria-hidden /> Staff portal
          </Link>
        </div>

        <section className="mb-12">
          <h2 className="text-xl font-bold text-primary mb-4">Pending contributions</h2>

          {contributions === null ? (
            <p className="text-sm text-text-secondary">Loading contributions...</p>
          ) : contributions.length === 0 ? (
            <p className="text-sm text-text-secondary">No pending contributions.</p>
          ) : (
            <div className="flex flex-col gap-3">
              {contributions.map((suggestion) => {
                const isLoading = actionLoading === suggestion.id;
                return (
                  <div
                    key={suggestion.id}
                    className="flex items-center gap-4 bg-bg border border-border rounded-lg px-5 py-4"
                  >
                    <div className="min-w-0 flex-1">
                      <p className="text-sm font-semibold text-text truncate">
                        {suggestion.title || "Untitled contribution"}
                      </p>
                      <p className="text-sm text-text-secondary truncate">
                        {suggestion.comment || ""}
                      </p>
                      <p className="text-xs text-text-muted mt-1">
                        {suggestion.authorName || "Unknown"} · {suggestion.courseName || "General"}
                      </p>
                    </div>
                    <div className="flex items-center gap-2 shrink-0">
                      <button
                        disabled={isLoading}
                        onClick={() => handleModerate(suggestion.id, "Approve")}
                        className="inline-flex items-center gap-1.5 border border-green-600 text-green-700 hover:bg-green-50 px-3 py-1.5 rounded text-sm font-medium transition-colors disabled:opacity-50"
                      >
                        <CheckmarkIcon aria-hidden /> Approve
                      </button>
                      <button
                        disabled={isLoading}
                        onClick={() => handleModerate(suggestion.id, "Reject")}
                        className="inline-flex items-center gap-1.5 border border-red-500 text-red-600 hover:bg-red-50 px-3 py-1.5 rounded text-sm font-medium transition-colors disabled:opacity-50"
                      >
                        <XMarkIcon aria-hidden /> Reject
                      </button>
                    </div>
                  </div>
                );
              })}
            </div>
          )}
        </section>

        <section>
          <h2 className="text-xl font-bold text-primary mb-4">Reported content</h2>

          <div className="flex flex-col gap-3">
            {mockReportedContent.map((report) => (
              <div
                key={report.id}
                className="flex items-start gap-4 bg-bg border border-border rounded-lg px-5 py-4"
              >
                <ExclamationmarkTriangleFillIcon
                  aria-hidden
                  className="text-red-500 mt-0.5 shrink-0"
                  fontSize="1.25rem"
                />
                <div className="min-w-0 flex-1">
                  <p className="text-sm font-semibold text-text truncate">
                    {reportCategoryLabels[report.category]}
                  </p>
                  <p className="text-sm text-text-secondary truncate">{report.description}</p>
                  <p className="text-xs text-text-muted mt-1">
                    {report.contactEmail ? `Contact: ${report.contactEmail} · ` : ""}
                    {new Date(report.createdAt).toLocaleDateString()}
                  </p>
                </div>
                <div className="flex items-center gap-2 shrink-0">
                  <button className="border border-border text-text-secondary hover:bg-surface px-3 py-1.5 rounded text-sm font-medium transition-colors">
                    Resolve
                  </button>
                  <button
                    aria-label="Delete report"
                    className="inline-flex items-center justify-center w-8 h-8 text-text-muted hover:text-red-600 hover:bg-surface rounded transition-colors"
                  >
                    <TrashIcon aria-hidden />
                  </button>
                </div>
              </div>
            ))}
          </div>
        </section>
      </main>
    </div>
  );
}