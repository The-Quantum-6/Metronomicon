import Navbar from "../components/Navbar";
import { useState, type FormEvent } from "react";
import { Link } from "react-router-dom";
import { apiUrl } from "../config";
import { ArrowLeftIcon, CheckmarkIcon, PlusIcon } from "@navikt/aksel-icons";

const inputStyle =
  "block w-full bg-surface-dark border border-border rounded p-2 text-text placeholder:text-placeholder focus:border-accent focus:outline-none";

type UserRole = "User" | "Admin" | "Root";

const ROLES: UserRole[] = ["User", "Admin", "Root"];

type User = {
  id: string;
  name: string;
  role: UserRole;
  email?: string;
};

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

function CreateCoursePanel() {
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
        body: JSON.stringify({ Create: { name, code, field, description } }),
      });
      if (!res.ok) throw new Error("create failed");
      setName("");
      setCode("");
      setField("");
      setDescription("");
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

function PermissionsPanel() {
  const [users, setUsers] = useState<User[]>([
    { id: "1", name: "John Doe", role: "User" },
    { id: "2", name: "John Doe", role: "User" },
    { id: "3", name: "John Doe", role: "User" },
  ]);

  const changeRole = (id: string, role: UserRole) => {
    setUsers(users.map((user) => (user.id === id ? { ...user, role } : user)));
  };

  return (
    <section className="bg-bg border border-border rounded-lg p-6">
      <h2 className="text-lg font-bold text-primary mb-4">Manage permissions</h2>

      <div className="flex flex-col gap-3">
        {users.map((user) => (
          <div key={user.id} className="bg-surface-dark border border-border rounded-lg px-4 py-3">
            <p className="text-sm font-semibold text-text truncate">{user.name}</p>
            <div className="flex flex-wrap gap-2 mt-3">
              {ROLES.map((role) => (
                <RoleButton
                  key={role}
                  label={role}
                  active={user.role === role}
                  onClick={() => changeRole(user.id, role)}
                />
              ))}
            </div>
          </div>
        ))}
      </div>
    </section>
  );
}

function RoleButton({
  label,
  active,
  onClick,
}: {
  label: string;
  active: boolean;
  onClick: () => void;
}) {
  const activeStyle = "bg-primary text-white";
  const inactiveStyle = "border border-border text-text-secondary hover:bg-surface";

  return (
    <button
      type="button"
      onClick={onClick}
      className={`inline-flex items-center gap-1 px-3 py-1.5 rounded text-xs font-medium transition-colors ${
        active ? activeStyle : inactiveStyle
      }`}
    >
      {active && <CheckmarkIcon aria-hidden />}
      {label.toUpperCase()}
    </button>
  );
}
