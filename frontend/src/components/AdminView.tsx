import { useEffect, useState } from "react";
import { apiUrl } from "../config";
import {
  CheckmarkIcon,
  XMarkIcon,
} from "@navikt/aksel-icons";

type Contribution = {
  id: string;
  title?: string;
  comment?: string;
  authorName?: string;
  courseName?: string;
  [key: string]: unknown;
};

export default function AdminView() {
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
    "TRANSFER_PERMS", ];

    type User = {
    id: string;
    name: string;
    permissions: string[]; };

    // TODO: Replace mock users with actual users
    const [users, setUsers] = useState<User[]>([
    { id: "1", name: "Hay", permissions: ["WRITE_TEXT", "SUGGEST_TEXT"] },
    { id: "2", name: "Jo", permissions: ["WRITE_FILE", "SUGGEST_FILE"] },
    { id: "3", name: "Kris", permissions: ["MODERATE_TEXT"] },
    { id: "4", name: "Lim", permissions: ["PAGE_ADMIN", "TRANSFER_PERMS"] },
    { id: "5", name: "Jørg", permissions: ["WRITE_TEXT", "MODERATE_FILE"] },
    ]);

    const displayedUsers = users;

    const togglePermission = (id: string, permission: string) => {
        setUsers((prev) =>
            prev.map((user) => {
                if (user.id !== id) return user;

        const hasPermission = user.permissions.includes(permission);
        return {
            ...user,
            permissions: hasPermission ? user.permissions.filter((item) => item !== permission) : [...user.permissions, permission],
        };
    })
    );
};

function fetchContributions() {
    fetch(apiUrl("contributions"), {
        credentials: "include",
    })
    .then((r) => {
        if (!r.ok)
            throw new Error(`Failed to load contributions (${r.status})`);
        return r.json() as Promise<Contribution[]>;
    })
    .then((data) => setContributions(data))
    .catch(() => setContributions([]));
}
useEffect(() => {
    fetchContributions();
}, []);

const handleModerate = async (
    contributionId: string,
    verdict: "Approve" | "Reject"
) => {
    setActionLoading(contributionId);

    try {
        const res = await fetch(apiUrl("contributions"), {
        method: "POST",
        headers: {
            "Content-Type": "application/json",
        },
        credentials: "include",
        body: JSON.stringify({
            Moderate: {
            contribution_id: contributionId,
            verdict,
        },
        }),
    });
    if (!res.ok) {
        throw new Error("Moderation failed");
    }
    
    setContributions((prev) => prev ? prev.filter((c) => c.id !== contributionId): []);
    } catch {
        alert("Failed to process contribution");
    } finally {
        setActionLoading(null);
    }
    };

    return (
    <div className="grid grid-cols-1 lg:grid-cols-2 gap-6 mt-8">

      {/* Pending contributions */}
        <section className="bg-white border border-[#DAD8D6] rounded-2xl p-5">
        <h2 className="text-xl font-semibold text-[#1A1F3A] mb-4">Pending contributions</h2>

        {contributions === null ? (
            <p className="text-sm text-[#6B6B5A]">
            Loading contributions...
            </p>
        ) : contributions.length === 0 ? (
            <p className="text-sm text-[#6B6B5A]">
            No pending contributions.
            </p>
        ) : (
            <div className="flex flex-col gap-3">
            {contributions.map((suggestion) => {
                const isLoading =
                actionLoading === suggestion.id;

                return (
                <div
                    key={suggestion.id}
                    className="border border-[#DAD8D6] rounded-xl p-4"
                >
                    <p className="font-semibold text-[#1A1F3A]">
                    {suggestion.title ||
                        "Untitled contribution"}
                    </p>

                    <p className="text-sm text-[#6B6B5A] mt-1">
                    {suggestion.comment || ""}
                    </p>

                    <p className="text-xs text-[#6B6B5A] mt-2">
                    {suggestion.authorName || "Unknown"} ·{" "}
                    {suggestion.courseName || "General"}
                    </p>

                    <div className="flex gap-2 mt-4">
                    <button
                        disabled={isLoading}
                        onClick={() =>
                        handleModerate(
                            suggestion.id,
                            "Approve"
                        )
                        }
                        className="inline-flex items-center gap-1.5 border border-green-600 text-green-700 hover:bg-green-50 px-3 py-2 rounded transition-colors disabled:opacity-50"
                    >
                        <CheckmarkIcon aria-hidden />
                        Approve
                        </button>

                    <button
                        disabled={isLoading}
                        onClick={() =>
                        handleModerate(
                            suggestion.id,
                            "Reject"
                        )
                        } 
                        className="inline-flex items-center gap-1.5 border border-red-500 text-red-600 hover:bg-red-50 px-3 py-2 rounded transition-colors disabled:opacity-50"
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
            <p className="text-sm text-[#6B6B5A]">
                No users found.
            </p>
            ) : (
            displayedUsers.map((user) => {
                const isOpen = openUserId === user.id;
                const visiblePermissions = permissions;

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
                        {visiblePermissions.length === 0 ? (
                        <p className="text-sm text-[#6B6B5A]">
                            No matching permissions.
                        </p>
                        ) : (
                        <div className="flex flex-wrap gap-2">
                            {visiblePermissions.map((permission) => {
                            const hasPermission = user.permissions.includes(permission);
                            return (
                                <button
                                key={`${user.id}-${permission}`}
                                onClick={() => togglePermission(user.id, permission)}
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
                        )}
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
