import Navbar from "../components/Navbar";
import { useState, type FormEvent, type ReactNode } from "react";
import { Link } from "react-router-dom";
import { apiUrl } from "../config";
import { ArrowLeftIcon, CheckmarkIcon, PlusIcon } from "@navikt/aksel-icons";

type Permissions = {
  moderator: boolean;
  pageAdmin: boolean;
  markOfficial: boolean;
  superuser: boolean;
};

const INPUT_CLASSES =
  "block w-full bg-surface-dark border border-border rounded p-2 text-text placeholder:text-placeholder focus:border-accent focus:outline-none";

/// Page staff use to publish new course pages and manage who else has staff access.
export default function StaffPortal() {
  return (
    <div className="min-h-screen bg-surface-dark text-text">
      <Navbar />
      <main className="container mx-auto max-w-5xl p-6">
        <Link
          to="/profile"
          className="inline-flex items-center gap-1.5 text-text-secondary hover:text-text transition-colors mb-4"
        >
          <ArrowLeftIcon aria-hidden /> Back
        </Link>

        <h1 className="text-2xl font-bold text-primary mb-6">Staff portal</h1>

        <div className="grid grid-cols-1 lg:grid-cols-2 gap-6 items-start">
          <CreateCoursePanel />
          <PermissionsPanel />
        </div>
      </main>
    </div>
  );
}

/// Form for publishing a new course page. Posts straight to the same
/// `courses/create` endpoint the admin course list uses.
function CreateCoursePanel() {
  const [name, setName] = useState("");
  const [code, setCode] = useState("");
  const [intro, setIntro] = useState("");
  const [credits, setCredits] = useState("");
  const [creating, setCreating] = useState(false);

  const handleSubmit = async (e: FormEvent) => {
    e.preventDefault();
    setCreating(true);
    try {
      const res = await fetch(apiUrl("courses/create"), {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        // Credits isn't sent - the backend's course model has no column for it yet.
        body: JSON.stringify({ name, code, content: intro }),
      });
      if (!res.ok) throw new Error("create failed");
      setName("");
      setCode("");
      setIntro("");
      setCredits("");
    } catch {
      alert("Create failed");
    } finally {
      setCreating(false);
    }
  };

  return (
    <section className="bg-bg border border-border rounded-lg p-6">
      <h2 className="text-lg font-bold text-primary mb-4">Create new course page</h2>
      <form onSubmit={handleSubmit} className="space-y-4">
        <FormField label="Course code">
          <input
            className={INPUT_CLASSES}
            placeholder="e.g. DATA1700"
            value={code}
            onChange={(e) => setCode(e.target.value)}
            required
          />
        </FormField>

        <FormField label="Course name">
          <input
            className={INPUT_CLASSES}
            placeholder="e.g. Webutvikling"
            value={name}
            onChange={(e) => setName(e.target.value)}
            required
          />
        </FormField>

        <FormField label="Student introduction">
          <textarea
            className={`${INPUT_CLASSES} min-h-28`}
            placeholder="Write a student-facing overview — what it covers, difficulty, why it's valuable…"
            value={intro}
            onChange={(e) => setIntro(e.target.value)}
          />
        </FormField>

        <FormField label="Credits (ECTS)">
          <input
            type="number"
            className={INPUT_CLASSES}
            placeholder="10"
            value={credits}
            onChange={(e) => setCredits(e.target.value)}
          />
        </FormField>

        <button
          type="submit"
          disabled={creating}
          className="inline-flex items-center gap-1.5 bg-accent hover:bg-accent-dark text-white px-4 py-2 rounded transition disabled:opacity-60"
        >
          <PlusIcon aria-hidden /> Create course
        </button>
      </form>
    </section>
  );
}

/// Labelled wrapper around a single form input, so each field is one line below.
function FormField({ label, children }: { label: string; children: ReactNode }) {
  return (
    <div>
      <label className="block text-sm font-medium text-text-secondary mb-1">{label}</label>
      {children}
    </div>
  );
}

const PLACEHOLDER_STAFF_COUNT = 3;

/// Lets staff grant/revoke roles for other staff members. There's no backend
/// support for these granular permissions yet, so this is placeholder rows
/// whose toggles only live in local state and reset on reload.
function PermissionsPanel() {
  const [staffPermissions, setStaffPermissions] = useState<Permissions[]>(() =>
    Array.from({ length: PLACEHOLDER_STAFF_COUNT }, () => ({
      moderator: false,
      pageAdmin: false,
      markOfficial: false,
      superuser: false,
    }))
  );

  const togglePermission = (index: number, key: keyof Permissions) => {
    setStaffPermissions((prev) =>
      prev.map((perms, i) => (i === index ? { ...perms, [key]: !perms[key] } : perms))
    );
  };

  return (
    <section className="bg-bg border border-border rounded-lg p-6">
      <h2 className="text-lg font-bold text-primary mb-4">Manage permissions</h2>

      <div className="flex flex-col gap-3">
        {staffPermissions.map((perms, index) => (
          <div key={index} className="bg-surface-dark border border-border rounded-lg px-4 py-3">
            <p className="text-sm font-semibold text-text truncate">John Doe</p>
            <div className="flex flex-wrap gap-2 mt-3">
              <PermissionToggle
                label="Moderator"
                active={perms.moderator}
                onClick={() => togglePermission(index, "moderator")}
              />
              <PermissionToggle
                label="Page admin"
                active={perms.pageAdmin}
                onClick={() => togglePermission(index, "pageAdmin")}
              />
              <PermissionToggle
                label="Mark official"
                active={perms.markOfficial}
                onClick={() => togglePermission(index, "markOfficial")}
              />
              <PermissionToggle
                label="Superuser"
                active={perms.superuser}
                onClick={() => togglePermission(index, "superuser")}
              />
            </div>
          </div>
        ))}
      </div>
    </section>
  );
}

/// One role chip: filled with a checkmark when active, outlined when not.
function PermissionToggle({
  label,
  active,
  onClick,
}: {
  label: string;
  active: boolean;
  onClick: () => void;
}) {
  const className = active
    ? "inline-flex items-center gap-1 bg-primary text-white px-3 py-1.5 rounded text-xs font-medium transition-colors"
    : "inline-flex items-center gap-1 border border-border text-text-secondary hover:bg-surface px-3 py-1.5 rounded text-xs font-medium transition-colors";

  return (
    <button type="button" onClick={onClick} className={className}>
      {active && <CheckmarkIcon aria-hidden />}
      {label.toUpperCase()}
    </button>
  );
}
