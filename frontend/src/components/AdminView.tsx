import { useEffect, useState } from "react";
import { useParams } from "react-router-dom";
import { apiUrl } from "../config";
import { CheckmarkIcon, XMarkIcon } from "@navikt/aksel-icons";

type Report = {
  aggregate_id: string;
  target: string | null;
  title: string;
  description: string;
  contact_email: string | null;
  status: string;
};

type Contribution = {
  aggregate_id: string;
  course_id: string;
  contribution: any;
  status: string;
};

type UserPerm = {
  user_id: string;
  name?: string;
  perms: number;
};

const PERMISSION_FLAGS: Record<string, number> = {
  READ: 1 << 1,           // 2
  WRITE_TEXT: 1 << 2,     // 4
  WRITE_FILE: 1 << 3,     // 8
  SUGGEST_TEXT: 1 << 4,   // 16
  SUGGEST_FILE: 1 << 5,   // 32
  MODERATE_TEXT: 1 << 6,  // 64
  MODERATE_FILE: 1 << 7,  // 128
  PAGE_ADMIN: 1 << 8,     // 256
  TRANSFER_PERMS: 1 << 9, // 512
};

function parseContribution(contribution: any) {
  if (!contribution) return { type: "Unknown", title: "Empty contribution", details: null };

  const domain = contribution.Text || contribution.File || contribution;

  if (domain.AddLink) {
    return {
      type: "Link",
      title: domain.AddLink.label || "New Link",
      details: domain.AddLink.url,
    };
  }

  if (domain.AddFaqEntry) {
    return {
      type: "FAQ Entry",
      title: domain.AddFaqEntry.question,
      details: domain.AddFaqEntry.answer,
    };
  }

  const firstKey = Object.keys(domain)[0] || "Contribution";
  const firstValue = domain[firstKey];

  return {
    type: firstKey,
    title: firstValue?.title || firstValue?.name || firstValue?.question || firstValue?.label || "Untitled contribution",
    details: firstValue?.details || firstValue?.answer || firstValue?.url || firstValue?.text || null,
  };
}

export default function AdminView() {
  const params = useParams<{ courseId?: string; id?: string; course_id?: string }>();
  const courseId = params.courseId || params.id || params.course_id;

  const [openUserId, setOpenUserId] = useState<string | null>(null);
  const [contributions, setContributions] = useState<Contribution[] | null>(null);
  const [actionLoading, setActionLoading] = useState<string | null>(null);

  const [users, setUsers] = useState<UserPerm[] | null>(null);
  const [updatingPerm, setUpdatingPerm] = useState<string | null>(null);

  const [reports, setReports] = useState<Report[]>([]);
  const [reportsLoading, setReportsLoading] = useState(true);

  const [isContributionsOpen, setIsContributionsOpen] = useState(true);
  const [isReportsOpen, setIsReportsOpen] = useState(true);

  async function loadReports() {
    try {
      const res = await fetch(apiUrl("reports"), { credentials: "include" });
      const data = await res.json();
      setReports(data);
    } catch {
      setReports([]);
    } finally {
      setReportsLoading(false);
    }
  }

  async function updateReport(id: string, action: "ResolveReport" | "ReopenReport") {
    await fetch(apiUrl("reports"), {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      credentials: "include",
      body: JSON.stringify({ [action]: { issue_id: id } }),
    });
    loadReports();
  }

  useEffect(() => {
    if (!courseId) return;
    fetch(`${apiUrl("contributions")}?course_id=${encodeURIComponent(courseId)}`, { credentials: "include" })
      .then((r) => (r.ok ? r.json() : []))
      .then((data) => setContributions(data))
      .catch(() => setContributions([]));

    fetch(`${apiUrl("permissions")}/all?course_id=${encodeURIComponent(courseId)}`, { credentials: "include" })
      .then((r) => {
        if (!r.ok) throw new Error("Kunne ikke hente rettigheter");
        return r.json() as Promise<UserPerm[]>;
      })
      .then((data) => setUsers(data))
      .catch((err) => {
        console.error("Feil ved henting av permissions:", err);
        setUsers([]);
      });

    loadReports();
  }, [courseId]);

  const togglePermission = async (userId: string, flagBit: number) => {
    if (!courseId) return;

    const user = users?.find((u) => u.user_id === userId);
    if (!user) return;

    const newPerms = user.perms ^ flagBit;

    setUpdatingPerm(`${userId}-${flagBit}`);

    setUsers((prev) =>
      prev ? prev.map((u) => (u.user_id === userId ? { ...u, perms: newPerms } : u)) : []
    );

    try {
      const res = await fetch(`${apiUrl("permissions")}?course_id=${encodeURIComponent(courseId)}`, {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        credentials: "include",
        body: JSON.stringify({
          user_id: userId,
          course_id: courseId,
          permissions: newPerms,
        }),
      });

      if (!res.ok) throw new Error("Serverfeil ved oppdatering av rettighet");
    } catch (err) {
      console.error("Feil ved lagring:", err);
      setUsers((prev) =>
        prev ? prev.map((u) => (u.user_id === userId ? { ...u, perms: user.perms } : u)) : []
      );
    } finally {
      setUpdatingPerm(null);
    }
  };

  const handleModerate = async (contributionId: string, action: string) => {
    setActionLoading(contributionId);

    const payload = {
      Moderate: {
        contribution_id: contributionId,
        verdict: action,
      },
    };

    try {
      const res = await fetch(apiUrl("contributions"), {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        credentials: "include",
        body: JSON.stringify(payload),
      });

      if (!res.ok) throw new Error("Feil under moderering");

      setContributions((prev) =>
        prev ? prev.filter((c) => c.aggregate_id !== contributionId) : []
      );
    } catch (err) {
      console.error(err);
      alert("Kunne ikke behandle forslaget.");
    } finally {
      setActionLoading(null);
    }
  };

  const courseReports = reports.filter((report) => report.target && report.target.startsWith("Course"));
  const openReports = courseReports.filter((report) => report.status !== "Resolved");
  const resolvedReports = courseReports.filter((report) => report.status === "Resolved");

  return (
    <>
    <ModerationGuidelines />
    <div className="grid grid-cols-1 lg:grid-cols-2 gap-6 mt-6">
      {/* Pending contributions */}
      <div className="flex flex-col gap-6">
        <section className="bg-white border border-[#DAD8D6] rounded-2xl overflow-hidden">
          <div
            onClick={() => setIsContributionsOpen(!isContributionsOpen)}
            className={`relative flex justify-between items-center w-full text-left p-4 cursor-pointer transition-colors ${
              isContributionsOpen ? "after:absolute after:left-5 after:right-5" : ""}`}>
            <h2 className="text-xl font-semibold text-[#1A1F3A]">Pending contributions</h2>
            <svg
              xmlns="http://www.w3.org/2000/svg"
              viewBox="0 0 24 24"
              fill="none"
              stroke="currentColor"
              strokeWidth="2"
              className={`w-5 h-5 text-[#6B6B5A] transition-transform duration-200 ${
                isContributionsOpen ? "rotate-180" : ""
              }`}>
              <path strokeLinecap="round" strokeLinejoin="round" d="M19 9l-7 7-7-7" />
            </svg>
          </div>

          {isContributionsOpen && (
            <div className="p-4">
              {contributions === null ? (
                <p className="text-sm text-[#6B6B5A]">Loading contributions...</p>
              ) : contributions.length === 0 ? (
                <p className="text-sm text-[#6B6B5A]">No pending contributions.</p>
              ) : (
                <div className="flex flex-col gap-3">
                  {contributions.map((suggestion) => {
                    const isLoading = actionLoading === suggestion.aggregate_id;
                    const { type, title, details } = parseContribution(suggestion.contribution);

                    return (
                      <div
                        key={suggestion.aggregate_id}
                        className="border border-[#DAD8D6] rounded-xl p-4 bg-white"
                      >
                        <div className="flex justify-between items-start gap-2">
                          <div className="flex items-center gap-2">
                            <span className="text-xs px-2 py-0.5 rounded font-medium bg-blue-50 text-blue-700 border border-blue-200">
                              {type}
                            </span>
                            <p className="font-semibold text-[#1A1F3A]">{title}</p>
                          </div>
                          <span className="text-xs px-2 py-0.5 rounded border border-[#DAD8D6] bg-gray-50 text-[#6B6B5A] shrink-0">
                            {suggestion.status}
                          </span>
                        </div>

                        {details && (
                          <div className="mt-2 text-sm text-[#4A4D57] bg-gray-50 p-2.5 rounded-lg border border-[#EAE8E6]">
                            {type === "Link" ? (
                              <a
                                href={details}
                                target="_blank"
                                rel="noreferrer"
                                className="text-blue-600 hover:underline break-all"
                              >
                                {details}
                              </a>
                            ) : (
                              <p className="whitespace-pre-line">{details}</p>
                            )}
                          </div>
                        )}

                        <div className="flex gap-2 mt-4">
                          <button
                            disabled={isLoading}
                            onClick={() => handleModerate(suggestion.aggregate_id, "Approve")}
                            className="inline-flex items-center gap-1.5 border border-green-600 text-green-700 hover:bg-green-50 px-3 py-1.5 text-sm font-medium rounded-lg transition-colors disabled:opacity-50"
                          >
                            <CheckmarkIcon aria-hidden />
                            Approve
                          </button>

                          <button
                            disabled={isLoading}
                            onClick={() => handleModerate(suggestion.aggregate_id, "Reject")}
                            className="inline-flex items-center gap-1.5 border border-red-500 text-red-600 hover:bg-red-50 px-3 py-1.5 text-sm font-medium rounded-lg transition-colors disabled:opacity-50"
                          >
                            <XMarkIcon aria-hidden />
                            Reject
                          </button>
                        </div>
                      </div>
                    );
                  })}
                </div>
              )}
            </div>
          )}
        </section>
        <section className="bg-white border border-[#DAD8D6] rounded-2xl overflow-hidden">
          <div
            onClick={() => setIsReportsOpen(!isReportsOpen)}
            className={`relative flex justify-between items-center w-full text-left p-4 cursor-pointer ${
              isReportsOpen ? "after:absolute after:left-5 after:right-5" : ""}`}>
            <h2 className="text-xl font-semibold text-[#1A1F3A]">Reported content</h2>
            <svg
              xmlns="http://www.w3.org/2000/svg"
              viewBox="0 0 24 24"
              fill="none"
              stroke="currentColor"
              strokeWidth="2"
              className={`w-5 h-5 text-[#6B6B5A] transition-transform duration-200 ${
                isReportsOpen ? "rotate-180" : ""
              }`}
            >
              <path strokeLinecap="round" strokeLinejoin="round" d="M19 9l-7 7-7-7" />
            </svg>
          </div>

          {isReportsOpen && (
            <div className="p-5 pt-4">
              {reportsLoading ? (
                <p className="text-sm text-[#6B6B5A]">Loading reports...</p>
              ) : courseReports.length === 0 ? (
                <p className="text-sm text-[#6B6B5A]">No reports.</p>
              ) : (
                <div className="flex flex-col gap-3">
                  <div className={`flex flex-col gap-3 ${resolvedReports.length > 0 ? "border-b border-[#DAD8D6] pb-5" : ""}`}>
                    {openReports.length === 0 && (
                      <p className="text-sm text-[#6B6B5A]">No open reports.</p>
                    )}
                    {openReports.map((report) => (
                      <div
                        key={report.aggregate_id}
                        className="flex items-start justify-between gap-4 bg-white border border-[#DAD8D6] rounded-xl px-5 py-4"
                      >
                        <div className="min-w-0 flex flex-col gap-1">
                          <div className="flex items-center">
                            <span className="text-[11px] font-semibold uppercase tracking-wider px-2 py-0.5 rounded bg-emerald-50 text-emerald-700 border border-emerald-200">
                              Status: Open
                            </span>
                          </div>
                          <h2 className="text-base font-semibold text-[#1A1F3A] mt-1">{report.title}</h2>
                          <p className="text-sm text-[#4A4D57]">{report.description}</p>
                          <p className="text-xs text-[#6B6B5A] mt-1">
                            {report.contact_email ? ` Contact: ${report.contact_email}` : ""}
                          </p>
                        </div>

                        <button
                          onClick={() => updateReport(report.aggregate_id, "ResolveReport")}
                          className="bg-[#1A1F3A] text-white hover:opacity-90 px-3 py-1.5 rounded-lg text-sm shrink-0 font-medium transition-colors mt-1"
                        >
                          Resolve
                        </button>
                      </div>
                    ))}
                  </div>

                  {resolvedReports.length > 0 && (
                    <div className="flex flex-col gap-3 pt-1">
                      <h2 className="text-xs font-semibold uppercase tracking-wider text-[#6B6B5A]">
                        Resolved ({resolvedReports.length})
                      </h2>
                      {resolvedReports.map((report) => (
                        <div
                          key={report.aggregate_id}
                          className="flex items-start justify-between gap-4 bg-gray-50 border border-[#DAD8D6] rounded-xl px-5 py-4 opacity-60 transition-opacity hover:opacity-90"
                        >
                          <div className="min-w-0 flex flex-col gap-1">
                            <div className="flex items-center">
                              <span className="text-[11px] font-semibold uppercase tracking-wider px-2 py-0.5 rounded bg-gray-100 text-[#6B6B5A] border border-[#DAD8D6]">
                                Status: Resolved
                              </span>
                            </div>
                            <h3 className="text-base font-semibold text-[#4A4D57] line-through mt-1">{report.title}</h3>
                            <p className="text-sm text-[#4A4D57] line-through">{report.description}</p>
                            <p className="text-xs text-[#6B6B5A] mt-1">
                              {report.contact_email ? `Contact info: ${report.contact_email}` : ""}
                            </p>
                          </div>

                          <button
                            onClick={() => updateReport(report.aggregate_id, "ReopenReport")}
                            className="border border-[#DAD8D6] text-[#4A4D57] hover:bg-white px-3 py-1.5 rounded-lg text-sm shrink-0 font-medium transition-colors mt-1"
                          >
                            Reopen
                          </button>
                        </div>
                      ))}
                    </div>
                  )}
                </div>
              )}
            </div>
          )}
        </section>
      </div>

       {/* Permissions */}
      <section className="bg-white border border-[#DAD8D6] rounded-2xl p-5 h-[640px] flex flex-col">
        <h2 className="text-xl font-semibold text-[#1A1F3A] mb-4">
          Manage permissions
        </h2>

        <div className="flex-1 overflow-y-auto space-y-3 pr-1">
          {users === null ? (
            <p className="text-sm text-[#6B6B5A]">Loading users...</p>
          ) : users.length === 0 ? (
            <p className="text-sm text-[#6B6B5A]">No users found for this course.</p>
          ) : (
            users.map((user) => {
              const isOpen = openUserId === user.user_id;

              return (
                <div key={user.user_id} className="border border-[#DAD8D6] rounded-xl p-4 bg-white flex flex-col justify-between">
                  <div>
                    <button
                      onClick={() => setOpenUserId(isOpen ? null : user.user_id)}
                      className="relative flex w-full items-center justify-between text-left"
                    >
                      {/* Viser brukernavn her */}
                      <span className="font-semibold text-[#1A1F3A]">
                        {user.name || "bruker"}
                      </span>
                      <span className="text-sm text-[#6B6B5A]">
                        {isOpen ? "Hide" : "View"} perms
                      </span>
                    </button>

                    {isOpen && (
                      <div className="mt-3 border-t border-[#DAD8D6] pt-3">
                        <div className="flex flex-wrap gap-2">
                          {Object.entries(PERMISSION_FLAGS).map(([permName, flagBit]) => {
                            const hasPermission = (user.perms & flagBit) !== 0;
                            const isBtnLoading = updatingPerm === `${user.user_id}-${flagBit}`;

                            return (
                              <button
                                key={`${user.user_id}-${permName}`}
                                disabled={isBtnLoading}
                                onClick={() => togglePermission(user.user_id, flagBit)}
                                className={`border rounded-lg px-3 py-2 text-sm transition-colors disabled:opacity-50 ${
                                  hasPermission
                                    ? "border-green-600 bg-green-50 text-green-700 font-medium"
                                    : "border-[#DAD8D6] text-[#1A1F3A] hover:bg-gray-100"
                                }`}
                              >
                                {permName}
                              </button>
                            );
                          })}
                        </div>

                        {/* ID */}
                        <div className="mt-4 pt-2 border-t border-[#EAE8E6] text-xs font-mono text-[#6B6B5A] select-all">
                          ID: {user.user_id}
                        </div>
                      </div>
                    )}
                  </div>
                </div>
              );
            })
          )}
        </div>
      </section>
    </div>
    </>
  );
}

function ModerationGuidelines() {
  const [open, setOpen] = useState(true);

  return (
    <section className="bg-white border border-[#DAD8D6] rounded-2xl p-5 mt-8">
      <button
        onClick={() => setOpen(!open)}
        className="flex w-full items-center justify-between text-left"
      >
        <h2 className="text-xl font-semibold text-[#1A1F3A]">Moderation guidelines</h2>
        <span className="text-sm text-[#6B6B5A]">{open ? "Hide" : "Show"}</span>
      </button>

      {open && (
        <div className="mt-4 space-y-4 text-sm text-[#4A4D57]">
          <p>
            You keep Metronomicon useful and trustworthy. Users suggest changes to course pages,
            and you decide what goes live. Your work happens in your queue: contributions and
            reports.
          </p>

          <div>
            <h3 className="font-semibold text-[#1A1F3A] mb-1">Contributions: Approve or Reject</h3>
            <ul className="list-disc pl-5 space-y-1">
              <li>Each is a link, FAQ entry, or project idea.</li>
              <li><b>Approve</b> if it's relevant, accurate, helpful, and respectful.</li>
              <li><b>Reject</b> if it's off-topic, wrong, spam/ads, offensive, or copyright-infringing.</li>
              <li>
                If a contribution reproduces copyrighted material (e.g., full documents, slides, or
                text lifted from another source), reject it and prefer a link to the original
                instead.
              </li>
              <li>In doubt? Open the link or read the answer first, or leave it for a second opinion.</li>
            </ul>
          </div>

          <div>
            <h3 className="font-semibold text-[#1A1F3A] mb-1">Reports: Resolve</h3>
            <ul className="list-disc pl-5 space-y-1">
              <li>Categories: bug, copyright/sensitive, policy, other.</li>
              <li><b>Resolve</b> = you've handled the underlying issue, or determined there wasn't one.</li>
              <li>
                Copyright reports should be resolved by removing the infringing content, not just
                closing the report.
              </li>
            </ul>
          </div>

          <div>
            <h3 className="font-semibold text-[#1A1F3A] mb-1">Principles</h3>
            <p>
              Be consistent and fair, assume good faith but verify, protect students over volume,
              and ask a course admin when unsure.
            </p>
          </div>
        </div>
      )}
    </section>
  );
}