import Navbar from "../components/Navbar";
import {
  CheckmarkIcon,
  XMarkIcon,
  TrashIcon,
  ExclamationmarkTriangleFillIcon,
  PersonIcon
} from "@navikt/aksel-icons";
import {
  mockSuggestions,
  mockReportedContent,
  formatSuggestion,
  reportCategoryLabels,
} from "../data/mockData";

export default function Profile() {
  return (
    <div className="min-h-screen bg-surface-dark text-text">
      <Navbar />
      <main className="container mx-auto max-w-4xl p-6">
        <div className="flex items-center gap-4 mb-10">
          <div className="inline-flex items-center justify-center w-10 h-10 rounded-full shrink-0 bg-surface-light text-text border border-border no-underline transition-colors hover:bg-surface">
            <PersonIcon aria-hidden fontSize="1.375rem" />
          </div>
        </div>

        <section className="mb-12">
          <h2 className="text-xl font-bold text-primary mb-4">Pending contributions</h2>

          <div className="flex flex-col gap-3">
            {mockSuggestions.map((suggestion) => {
              const { title, detail } = formatSuggestion(suggestion.kind);
              return (
                <div
                  key={suggestion.id}
                  className="flex items-center gap-4 bg-bg border border-border rounded-lg px-5 py-4"
                >
                  <div className="min-w-0 flex-1">
                    <p className="text-sm font-semibold text-text truncate">{title}</p>
                    <p className="text-sm text-text-secondary truncate">{detail}</p>
                    <p className="text-xs text-text-muted mt-1">
                      {suggestion.authorName} · {suggestion.courseName}
                    </p>
                  </div>
                  <div className="flex items-center gap-2 shrink-0">
                    <button className="inline-flex items-center gap-1.5 border border-green-600 text-green-700 hover:bg-green-50 px-3 py-1.5 rounded text-sm font-medium transition-colors">
                      <CheckmarkIcon aria-hidden /> Approve
                    </button>
                    <button className="inline-flex items-center gap-1.5 border border-red-500 text-red-600 hover:bg-red-50 px-3 py-1.5 rounded text-sm font-medium transition-colors">
                      <XMarkIcon aria-hidden /> Reject
                    </button>
                  </div>
                </div>
              );
            })}
          </div>
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