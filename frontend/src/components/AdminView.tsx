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

  const permissions = [
    "WRITE_TEXT",
    "WRITE_FILE",
    "SUGGEST_TEXT",
    "SUGGEST_FILE",
    "MODERATE_TEXT",
    "MODERATE_FILE",
    "PAGE_ADMIN",
    "TRANSFER_PERMS",
  ];

  type User = {
    id: string;
    name: string;
    permissions: string[];
  };

  const [users, setUsers] = useState<User[]>([
    { id: "1", name: "Hay", permissions: ["WRITE_TEXT", "SUGGEST_TEXT"] },
    { id: "2", name: "Jo", permissions: ["WRITE_FILE", "SUGGEST_FILE"] },
    { id: "3", name: "Kris", permissions: ["MODERATE_TEXT"] },
    { id: "4", name: "Lim", permissions: ["PAGE_ADMIN", "TRANSFER_PERMS"] },
    { id: "5", name: "Jørg", permissions: ["WRITE_TEXT", "MODERATE_FILE"] },
  ]);

  const togglePermission = (id: string, permission: string) => {
    setUsers((prev) =>
      prev.map((user) => {
        if (user.id !== id) return user;

        const hasPermission = user.permissions.includes(permission);
        return {
          ...user,
          permissions: hasPermission
            ? user.permissions.filter((item) => item !== permission)
            : [...user.permissions, permission],
        };
      })
    );
  };

  useEffect(() => {
    if (!courseId) {
      console.warn("Fant ingen courseId i URL-en!");
      return;
    }

    const endpoint = `${apiUrl("contributions")}?course_id=${encodeURIComponent(courseId)}`;

    fetch(endpoint, {
      credentials: "include",
    })
      .then((r) => {
        if (!r.ok) throw new Error(`Failed to load contributions (${r.status})`);
        return r.json() as Promise<Contribution[]>;
      })
      .then((data) => {
        setContributions(data);
      })
      .catch((err) => {
        console.error("Feil ved henting:", err);
        setContributions([]);
      });
  }, [courseId]);

    const handleModerate = async (
  contributionId: string,
  action: string
) => {
  setActionLoading(contributionId);


  const payload = {
    Moderate: {
      contribution_id: contributionId,
      verdict: action, 
    },
  };

  console.log("Sender moderering til backend:", payload);

  try {
    const res = await fetch(apiUrl("contributions"), {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      credentials: "include",
      body: JSON.stringify(payload),
    });

    if (!res.ok) {
      const errorText = await res.text();
      console.error(`Feil fra backend (${res.status}):`, errorText);
      throw new Error(`Klarte ikke å behandle forslaget (${res.status}): ${errorText}`);
    }
    setContributions((prev) =>
      prev ? prev.filter((c) => c.aggregate_id !== contributionId) : []
    );
  } catch (err) {
    console.error("Feil i handleModerate:", err);
    alert(`Feil ved moderering! Sjekk konsollen (F12) for mer detaljer.`);
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

                  <p className="text-xs text-[#6B6B5A] mt-3">
                    ID: {suggestion.aggregate_id}
                  </p>

                  <div className="flex gap-2 mt-4">
                    <button
                      disabled={isLoading}
                      onClick={() =>
                        handleModerate(suggestion.aggregate_id, "Approve")
                      }
                      className="inline-flex items-center gap-1.5 border border-green-600 text-green-700 hover:bg-green-50 px-3 py-1.5 text-sm font-medium rounded-lg transition-colors disabled:opacity-50"
                    >
                      <CheckmarkIcon aria-hidden />
                      Approve
                    </button>

                    <button
                      disabled={isLoading}
                      onClick={() =>
                        handleModerate(suggestion.aggregate_id, "Reject")
                      }
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
          {users.length === 0 ? (
            <p className="text-sm text-[#6B6B5A]">No users found.</p>
          ) : (
            users.map((user) => {
              const isOpen = openUserId === user.id;

              return (
                <div
                  key={user.id}
                  className="border border-[#DAD8D6] rounded-xl p-4"
                >
                  <button
                    onClick={() => setOpenUserId(isOpen ? null : user.id)}
                    className="relative flex w-full items-center justify-between text-left"
                  >
                    <span className="font-semibold text-[#1A1F3A]">
                      {user.name}
                    </span>
                    <span className="text-sm text-[#6B6B5A]">
                      {isOpen ? "Hide" : "View"} perms
                    </span>
                  </button>

                  {isOpen && (
                    <div className="mt-3 border-t border-[#DAD8D6] pt-3">
                      <div className="flex flex-wrap gap-2">
                        {permissions.map((permission) => {
                          const hasPermission =
                            user.permissions.includes(permission);
                          return (
                            <button
                              key={`${user.id}-${permission}`}
                              onClick={() =>
                                togglePermission(user.id, permission)
                              }
                              className={`border rounded-lg px-3 py-2 text-sm transition-colors ${
                                hasPermission
                                  ? "border-green-600 bg-green-50 text-green-700"
                                  : "border-[#DAD8D6] text-[#1A1F3A] hover:bg-gray-100"
                              }`}
                            >
                              {permission}
                            </button>
                          );
                        })}
                      </div>
                    </div>
                  )}
                </div>
              );
            })
          )}
        </div>
      </section>
    </div>
  );
}