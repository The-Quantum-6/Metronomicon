import { useEffect, useState } from "react";
import { useParams } from "react-router-dom";
import { apiUrl } from "../config";
import { CheckmarkIcon, XMarkIcon } from "@navikt/aksel-icons";

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

  return (
    <div className="grid grid-cols-1 lg:grid-cols-2 gap-6 mt-8">
      {/* Pending contributions */}
      <section className="bg-white border border-[#DAD8D6] rounded-2xl p-5">
        <h2 className="text-xl font-semibold text-[#1A1F3A] mb-4">
          Pending contributions
        </h2>

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
      </section>

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
  );
}