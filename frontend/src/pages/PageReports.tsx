import Navbar from "../components/Navbar";
import { useEffect, useState } from "react";
import { Link } from "react-router-dom";
import { apiUrl } from "../config";
import {
  CheckmarkIcon,
  ArrowUndoIcon,
  ArrowLeftIcon,
  ExclamationmarkTriangleFillIcon,
} from "@navikt/aksel-icons";

type Report = {
  aggregate_id: string;
  target: string | null;
  description: string | null;
  contact_email: string | null;
  status: string | null;
};

function parseReport(description: string | null): { category: string; body: string } {
  const match = description?.match(/^\[(.+?)\]\s*([\s\S]*)$/);
  if (match) {
    return { category: match[1], body: match[2] };
  }
  return { category: "Report", body: description ?? "" };
}

function formatTarget(target: string | null): string {
  if (!target) return "Site";
  return target.startsWith("Course") ? "Course" : target;
}

export default function PageReports() {
  const [reports, setReports] = useState<Report[] | null>(null);
  const [reportLoading, setReportLoading] = useState<string | null>(null);

  useEffect(() => {
    fetchReports();
  }, []);

  const fetchReports = () => {
    fetch(apiUrl("reports"), { credentials: "include" })
      .then((r) => {
        if (!r.ok) throw new Error(`Failed to load reports (${r.status})`);
        return r.json() as Promise<Report[]>;
      })
      .then((data) => setReports(data))
      .catch(() => setReports([]));
  };

  const handleReportStatus = async (issueId: string, action: "ResolveReport" | "ReopenReport") => {
    setReportLoading(issueId);
    try {
      const res = await fetch(apiUrl("reports"), {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        credentials: "include",
        body: JSON.stringify({ [action]: { issue_id: issueId } }),
      });

      if (!res.ok) throw new Error("Report update failed");

      const nextStatus = action === "ResolveReport" ? "Resolved" : "Open";
      setReports((prev) =>
        prev
          ? prev.map((r) => (r.aggregate_id === issueId ? { ...r, status: nextStatus } : r))
          : prev
      );
    } catch {
      alert("Failed to update report");
    } finally {
      setReportLoading(null);
    }
  };

  return (
    <div className="min-h-screen bg-surface-dark text-text">
      <Navbar />
      <main className="container mx-auto max-w-4xl p-6">
        <Link
          to="/"
          className="inline-flex items-center gap-1.5 text-text-secondary hover:text-text transition-colors mb-4"
        >
          <ArrowLeftIcon aria-hidden /> Back
        </Link>

        <h1 className="text-2xl font-bold text-primary mb-6">Reported content</h1>

        {reports === null ? (
          <p className="text-sm text-text-secondary">Loading reports...</p>
        ) : reports.length === 0 ? (
          <p className="text-sm text-text-secondary">No reports.</p>
        ) : (
          <div className="flex flex-col gap-3">
            {[...reports]
              .sort(
                (a, b) =>
                  Number(a.status === "Resolved") - Number(b.status === "Resolved")
              )
              .map((report) => {
              const { category, body } = parseReport(report.description);
              const isResolved = report.status === "Resolved";
              const isLoading = reportLoading === report.aggregate_id;
              return (
                <div
                  key={report.aggregate_id}
                  className={`flex items-start gap-4 bg-bg border border-border rounded-lg px-5 py-4 ${
                    isResolved ? "opacity-60" : ""
                  }`}
                >
                  <ExclamationmarkTriangleFillIcon
                    aria-hidden
                    className={`mt-0.5 shrink-0 ${isResolved ? "text-text-muted" : "text-red-500"}`}
                    fontSize="1.25rem"
                  />
                  <div className="min-w-0 flex-1">
                    <p className="text-sm font-semibold text-text truncate">{category}</p>
                    <p className="text-sm text-text-secondary truncate">{body}</p>
                    <p className="text-xs text-text-muted mt-1">
                      {formatTarget(report.target)}
                      {report.contact_email ? ` · Contact: ${report.contact_email}` : ""}
                      {` · ${isResolved ? "Resolved" : "Open"}`}
                    </p>
                  </div>
                  <div className="flex items-center gap-2 shrink-0">
                    {isResolved ? (
                      <button
                        disabled={isLoading}
                        onClick={() => handleReportStatus(report.aggregate_id, "ReopenReport")}
                        className="inline-flex items-center gap-1.5 border border-border text-text-secondary hover:bg-surface px-3 py-1.5 rounded text-sm font-medium transition-colors disabled:opacity-50"
                      >
                        <ArrowUndoIcon aria-hidden /> Reopen
                      </button>
                    ) : (
                      <button
                        disabled={isLoading}
                        onClick={() => handleReportStatus(report.aggregate_id, "ResolveReport")}
                        className="inline-flex items-center gap-1.5 border border-green-600 text-green-700 hover:bg-green-50 px-3 py-1.5 rounded text-sm font-medium transition-colors disabled:opacity-50"
                      >
                        <CheckmarkIcon aria-hidden /> Resolve
                      </button>
                    )}
                  </div>
                </div>
              );
            })}
          </div>
        )}
      </main>
    </div>
  );
}
