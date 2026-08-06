import Navbar from "../components/Navbar";
import { useEffect, useState } from "react";
import { Link } from "react-router-dom";
import { apiUrl } from "../config";
import { ArrowLeftIcon } from "@navikt/aksel-icons";

type Report = {
  aggregate_id: string;
  target: string | null;
  title: string;
  description: string;
  contact_email: string | null;
  status: string;
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
    } finally {
      setLoading(false);
    }
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


  const siteReports = reports.filter((report) => report.target === "Site");
  const openReports = siteReports.filter((report) => report.status !== "Resolved");
  const resolvedReports = siteReports.filter((report) => report.status === "Resolved");

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
        ) : siteReports.length === 0 ? (
          <p className="text-sm text-text-secondary">No reports.</p>
        ) : (
          <div className="flex flex-col gap-3">
            <div className={`flex flex-col gap-3 ${resolvedReports.length > 0 ? "border-b border-border pb-5" : ""}`}>
              {openReports.length === 0 && (
                <p className="text-sm text-text-muted">No open reports.</p>
              )}
              {openReports.map((report) => (
                <div
                  key={report.aggregate_id}
                  className="flex items-start justify-between gap-4 bg-bg border border-border rounded-xl px-5 py-4"
                >
                  <div className="min-w-0 flex flex-col gap-1">
                    <div className="flex items-center">
                      <span className="text-[11px] font-semibold uppercase tracking-wider px-2 py-0.5 rounded bg-emerald-500/10 text-emerald-600 border border-emerald-500/20">Status: Open</span>
                    </div>
                    <h2 className="text-base font-semibold text-text mt-1">{report.title}</h2>
                    <p className="text-sm text-text-secondary">{report.description}</p>
                    <p className="text-xs text-text-muted mt-1">
                      {report.contact_email ? ` · Contact: ${report.contact_email}` : ""}
                    </p>
                  </div>

                  <button
                    onClick={() => updateReport(report.aggregate_id, "ResolveReport")}
                    className="bg-[#1A1F3A] text-white hover:opacity-90 px-3 py-1.5 rounded text-sm shrink-0 font-medium transition-colors mt-1">
                    Resolve
                  </button>
                </div>
              ))}
            </div>
            {resolvedReports.length > 0 && (
              <div className="flex flex-col gap-3 pt-1">
                <h2 className="text-xs font-semibold uppercase tracking-wider text-text-muted">Resolved ({resolvedReports.length})</h2>
                {resolvedReports.map((report) => (
                  <div
                    key={report.aggregate_id}
                    className="flex items-start justify-between gap-4 bg-surface border border-border rounded-xl px-5 py-4 opacity-60 transition-opacity hover:opacity-90">
                    <div className="min-w-0 flex flex-col gap-1">
                      <div className="flex items-center">
                        <span className="text-[11px] font-semibold uppercase tracking-wider px-2 py-0.5 rounded bg-gray-500/10 text-text-muted border border-border">Status: Resolved</span>
                      </div>
                      <h3 className="text-base font-semibold text-text-secondary line-through mt-1">{report.title}</h3>
                      <p className="text-sm text-text-secondary line-through">{report.description}</p>
                      <p className="text-xs text-text-muted mt-1">{report.contact_email ? `Contact: ${report.contact_email}` : ""}</p>
                    </div>
                    <button
                      onClick={() => updateReport(report.aggregate_id, "ReopenReport")}
                      className="border border-border text-text-secondary hover:bg-bg px-3 py-1.5 rounded text-sm shrink-0 font-medium transition-colors mt-1"
                    >
                      Reopen
                    </button>
                  </div>
                ))}
              </div>
            )}
          </div>
        )}
      </main>
    </div>
  );
}
