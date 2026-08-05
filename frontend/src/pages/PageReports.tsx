import Navbar from "../components/Navbar";
import { useEffect, useState } from "react";
import { Link } from "react-router-dom";
import { apiUrl } from "../config";
import { ArrowLeftIcon } from "@navikt/aksel-icons";

type Report = {
  aggregate_id: string;
  target: string | null;
  description: string | null;
  contact_email: string | null;
  status: string | null;
};

function formatTarget(target: string | null): string {
  if (target && target.startsWith("Course")) {
    return "Course";
  }
  return "Site";
}

export default function PageReports() {
  const [reports, setReports] = useState<Report[]>([]);
  const [loading, setLoading] = useState(true);

  async function loadReports() {
    try {
      const res = await fetch(apiUrl("reports"), { credentials: "include" });
      const data = await res.json();
      setReports(data);
    } catch {
      setReports([]);
    }
    setLoading(false);
  }

  useEffect(() => {
    loadReports();
  }, []);

  async function updateReport(id: string, action: "ResolveReport" | "ReopenReport") {
    await fetch(apiUrl("reports"), {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      credentials: "include",
      body: JSON.stringify({ [action]: { issue_id: id } }),
    });
    loadReports();
  }

  const openReports = reports.filter((report) => report.status !== "Resolved");
  const resolvedReports = reports.filter((report) => report.status === "Resolved");
  const reportsToShow = [...openReports, ...resolvedReports];

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

        {loading ? (
          <p className="text-sm text-text-secondary">Loading reports...</p>
        ) : reportsToShow.length === 0 ? (
          <p className="text-sm text-text-secondary">No reports.</p>
        ) : (
          <div className="flex flex-col gap-3">
            {reportsToShow.map((report) => {
              const isResolved = report.status === "Resolved";

              return (
                <div
                  key={report.aggregate_id}
                  className="flex items-start justify-between gap-4 bg-bg border border-border rounded-lg px-5 py-4"
                >
                  <div className="min-w-0">
                    <p className="text-sm text-text">{report.description}</p>
                    <p className="text-xs text-text-muted mt-1">
                      {formatTarget(report.target)}
                      {report.contact_email ? ` · ${report.contact_email}` : ""}
                      {isResolved ? " · Resolved" : " · Open"}
                    </p>
                  </div>

                  {isResolved ? (
                    <button
                      onClick={() => updateReport(report.aggregate_id, "ReopenReport")}
                      className="border border-border text-text-secondary hover:bg-surface px-3 py-1.5 rounded text-sm shrink-0"
                    >
                      Reopen
                    </button>
                  ) : (
                    <button
                      onClick={() => updateReport(report.aggregate_id, "ResolveReport")}
                      className="bg-green-600 text-white hover:bg-green-700 px-3 py-1.5 rounded text-sm shrink-0"
                    >
                      Resolve
                    </button>
                  )}
                </div>
              );
            })}
          </div>
        )}
      </main>
    </div>
  );
}
