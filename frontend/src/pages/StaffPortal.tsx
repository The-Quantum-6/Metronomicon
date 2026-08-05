import Navbar from "../components/Navbar";
import { useEffect, useState, type FormEvent } from "react";
import { Link } from "react-router-dom";
import { apiUrl } from "../config";
import { ArrowLeftIcon, PlusIcon } from "@navikt/aksel-icons";

const inputStyle =
  "block w-full bg-surface-dark border border-border rounded p-2 text-text placeholder:text-placeholder focus:border-accent focus:outline-none";

type CourseListItem = {
  aggregate_id: string;
  name: string;
  code: string;
  field: string;
};

export default function StaffPortal() {
  const [refresh, setRefresh] = useState(0);

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
          <CreateCoursePanel onCreated={() => setRefresh((n) => n + 1)} />
          <ManageCoursesPanel refreshKey={refresh} />
        </div>
      </main>
    </div>
  );
}

function CreateCoursePanel({ onCreated }: { onCreated?: () => void }) {
  const [name, setName] = useState("");
  const [code, setCode] = useState("");
  const [field, setField] = useState("");
  const [description, setDescription] = useState("");
  const [creating, setCreating] = useState(false);

  const handleSubmit = async (e: FormEvent) => {
    e.preventDefault();
    setCreating(true);
    try {
      const res = await fetch(apiUrl("courses"), {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        credentials: "include",
        body: JSON.stringify({ Create: { name, code, field, description } }),
      });
      if (!res.ok) throw new Error("create failed");
      setName("");
      setCode("");
      setField("");
      setDescription("");
      onCreated?.();
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
        <div>
          <label className="block text-sm font-medium text-text-secondary mb-1">Course code</label>
          <input
            className={inputStyle}
            placeholder="e.g. DATA1700"
            value={code}
            onChange={(e) => setCode(e.target.value)}
            required
          />
        </div>

        <div>
          <label className="block text-sm font-medium text-text-secondary mb-1">Course name</label>
          <input
            className={inputStyle}
            placeholder="e.g. Webutvikling"
            value={name}
            onChange={(e) => setName(e.target.value)}
            required
          />
        </div>

        <div>
          <label className="block text-sm font-medium text-text-secondary mb-1">Course field</label>
          <input
            className={inputStyle}
            placeholder="Computer Science e.g. "
            value={field}
            onChange={(e) => setField(e.target.value)}
            required
          />
        </div>

        <div>
          <label className="block text-sm font-medium text-text-secondary mb-1">
            Description
          </label>
          <textarea
            className={`${inputStyle} min-h-28`}
            placeholder="Write a little intro to the course"
            value={description}
            onChange={(e) => setDescription(e.target.value)}
          />
        </div>


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

function ManageCoursesPanel({ refreshKey }: { refreshKey: number }) {
  const [active, setActive] = useState<CourseListItem[] | null>(null);
  const [unactive, setUnactive] = useState<CourseListItem[] | null>(null);
  const [busy, setBusy] = useState<string | null>(null);

  const load = () => {
    fetch(apiUrl("courses?status=active"), { credentials: "include" })
      .then((r) => (r.ok ? (r.json() as Promise<CourseListItem[]>) : []))
      .then((data) => setActive(data))
      .catch(() => setActive([]));
    fetch(apiUrl("courses?status=unactive"), { credentials: "include" })
      .then((r) => (r.ok ? (r.json() as Promise<CourseListItem[]>) : []))
      .then((data) => setUnactive(data))
      .catch(() => setUnactive([]));
  };

  useEffect(() => {
    load();
  }, [refreshKey]);

  const toggle = async (id: string, action: "activate" | "unactivate") => {
    setBusy(id);
    const command = action === "activate" ? "Activate" : "Unactivate";
    try {
      const res = await fetch(apiUrl(`courses/${id}/${action}`), {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        credentials: "include",
        body: JSON.stringify({ [command]: { course_id: id } }),
      });
      if (!res.ok) throw new Error("failed");
      load();
    } catch {
      alert(`Failed to ${action} course`);
    } finally {
      setBusy(null);
    }
  };

  const loading = active === null || unactive === null;
  const activeList = active ?? [];
  const unactiveList = unactive ?? [];

  return (
    <section className="bg-bg border border-border rounded-lg p-6">
      <h2 className="text-lg font-bold text-primary mb-4">Manage course status</h2>

      {loading ? (
        <p className="text-sm text-text-secondary">Loading courses...</p>
      ) : activeList.length === 0 && unactiveList.length === 0 ? (
        <p className="text-sm text-text-secondary">No courses yet.</p>
      ) : (
        <div className="flex flex-col gap-4">
          <CourseGroup
            label="Active"
            courses={activeList}
            actionLabel="Deactivate"
            busy={busy}
            onToggle={(id) => toggle(id, "unactivate")}
          />
          <CourseGroup
            label="Unactive"
            courses={unactiveList}
            actionLabel="Activate"
            busy={busy}
            onToggle={(id) => toggle(id, "activate")}
            primary
          />
        </div>
      )}
    </section>
  );
}

function CourseGroup({
  label,
  courses,
  actionLabel,
  busy,
  onToggle,
  primary,
}: {
  label: string;
  courses: CourseListItem[];
  actionLabel: string;
  busy: string | null;
  onToggle: (id: string) => void;
  primary?: boolean;
}) {
  if (courses.length === 0) return null;

  return (
    <div className="flex flex-col gap-2">
      <p className="text-xs font-semibold uppercase tracking-wide text-text-secondary">{label}</p>
      {courses.map((c) => (
        <div
          key={c.aggregate_id}
          className="flex items-center gap-3 bg-surface-dark border border-border rounded-lg px-4 py-3"
        >
          <div className="min-w-0 flex-1">
            <p className="text-sm font-semibold text-text truncate">{c.name}</p>
            <p className="text-xs text-text-secondary truncate">
              {c.code} · {c.field}
            </p>
          </div>
          <button
            type="button"
            disabled={busy === c.aggregate_id}
            onClick={() => onToggle(c.aggregate_id)}
            className={`inline-flex items-center gap-1 px-3 py-1.5 rounded text-xs font-medium transition-colors disabled:opacity-50 shrink-0 ${
              primary
                ? "bg-accent hover:bg-accent-dark text-white"
                : "border border-border text-text-secondary hover:bg-surface"
            }`}
          >
            {actionLabel}
          </button>
        </div>
      ))}
    </div>
  );
}
