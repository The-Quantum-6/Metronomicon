import Navbar from "../components/Navbar";
import { Link } from "react-router-dom";
import { useEffect, useState } from "react";
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
import {
  describeContribution,
  fileKeyOf,
  fileUrl,
  listContributions,
  moderateContribution,
  type Contribution,
  type ModerationVerdict,
} from "../api/contributions";

export default function Profile() {
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

        <PendingContributions />

        {/* TODO: reports are still mock data; wire up once the report endpoints land. */}
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

type QueueResult = { items: Contribution[] } | { error: string };

async function fetchPending(): Promise<QueueResult> {
  try {
    const all = await listContributions();
    return { items: all.filter((c) => c.status === "Proposed") };
  } catch (err) {
    console.error(err);
    return { error: "Could not load the moderation queue." };
  }
}

function PendingContributions() {
  const [pending, setPending] = useState<Contribution[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [busyId, setBusyId] = useState<string | null>(null);

  function apply(result: QueueResult) {
    if ("error" in result) {
      setError(result.error);
    } else {
      setPending(result.items);
      setError(null);
    }
    setLoading(false);
  }

  useEffect(() => {
    let cancelled = false;
    fetchPending().then((result) => {
      if (!cancelled) apply(result);
    });
    return () => {
      cancelled = true;
    };
  }, []);

  async function handleModerate(contributionId: string, verdict: ModerationVerdict) {
    setBusyId(contributionId);
    try {
      await moderateContribution(contributionId, verdict);
      // The projection and the process manager run after the command returns,
      // so refetch rather than assuming what the queue now looks like.
      apply(await fetchPending());
    } catch (err) {
      console.error(err);
      setError(
        verdict === "Approve"
          ? "Could not approve the contribution."
          : "Could not deny the contribution.",
      );
    } finally {
      setBusyId(null);
    }
  }

  return (
    <section className="mb-12">
      <h2 className="text-xl font-bold text-primary mb-4">Pending contributions</h2>

      {error && (
        <p role="alert" className="text-sm text-red-600 mb-3">
          {error}
        </p>
      )}

      {loading ? (
        <p className="text-sm text-text-secondary">Loading…</p>
      ) : pending.length === 0 ? (
        <p className="text-sm text-text-secondary">Nothing is waiting for review.</p>
      ) : (
        <div className="flex flex-col gap-3">
          {pending.map((contribution) => {
            const { title, detail } = describeContribution(contribution.contribution);
            const key = fileKeyOf(contribution.contribution);
            const busy = busyId === contribution.aggregate_id;

            return (
              <div
                key={contribution.aggregate_id}
                className="flex items-center gap-4 bg-bg border border-border rounded-lg px-5 py-4"
              >
                <div className="min-w-0 flex-1">
                  <p className="text-sm font-semibold text-text truncate">{title}</p>
                  <p className="text-sm text-text-secondary truncate">{detail}</p>
                  {contribution.comment && (
                    <p className="text-xs text-text-muted mt-1 truncate">
                      «{contribution.comment}»
                    </p>
                  )}
                  {key && (
                    <a
                      href={fileUrl(key)}
                      target="_blank"
                      rel="noreferrer"
                      className="text-xs text-accent hover:underline mt-1 inline-block"
                    >
                      Review the file before deciding
                    </a>
                  )}
                </div>
                <div className="flex items-center gap-2 shrink-0">
                  <button
                    disabled={busy}
                    onClick={() => handleModerate(contribution.aggregate_id, "Approve")}
                    className="inline-flex items-center gap-1.5 border border-green-600 text-green-700 hover:bg-green-50 disabled:opacity-50 px-3 py-1.5 rounded text-sm font-medium transition-colors"
                  >
                    <CheckmarkIcon aria-hidden /> Approve
                  </button>
                  <button
                    disabled={busy}
                    onClick={() => handleModerate(contribution.aggregate_id, "Deny")}
                    className="inline-flex items-center gap-1.5 border border-red-500 text-red-600 hover:bg-red-50 disabled:opacity-50 px-3 py-1.5 rounded text-sm font-medium transition-colors"
                  >
                    <XMarkIcon aria-hidden /> Deny
                  </button>
                </div>
              </div>
            );
          })}
        </div>
      )}
    </section>
  );
}