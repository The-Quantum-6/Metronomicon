import { useEffect, useRef, useState } from "react";
import { Link } from "react-router-dom";
import { HStack, Spacer, Button } from "@navikt/ds-react";
import { HouseIcon, PersonIcon } from "@navikt/aksel-icons";
import { apiUrl, apiBaseUrl } from "../config";

type User = {
  id: string;
  sub: string;
  name: string;
  role: "user" | "admin" | "root";
  email: string | null;
};

function Navbar() {
  const [user, setUser] = useState<User | null>(null);
  const [checked, setChecked] = useState(false);
  const [open, setOpen] = useState(false);
  const menuRef = useRef<HTMLDivElement>(null);
  const isAdmin = user?.role === "admin" || user?.role === "root";
  useEffect(() => {
    fetch(apiUrl("me"), { credentials: "include", redirect: "manual" })
      .then((r) => (r.ok ? (r.json() as Promise<User>) : null))
      .then((data) => setUser(data))
      .catch(() => setUser(null))
      .finally(() => setChecked(true));
  }, []);

  useEffect(() => {
    if (!open) return;
    const onClick = (e: MouseEvent) => {
      if (menuRef.current && !menuRef.current.contains(e.target as Node)) {
        setOpen(false);
      }
    };
    const onKey = (e: KeyboardEvent) => e.key === "Escape" && setOpen(false);
    document.addEventListener("mousedown", onClick);
    document.addEventListener("keydown", onKey);
    return () => {
      document.removeEventListener("mousedown", onClick);
      document.removeEventListener("keydown", onKey);
    };
  }, [open]);

  const handleLogout = () => {
    fetch(apiUrl("logout"), { method: "POST", credentials: "include" }).catch(() => {});
    setUser(null);
    setOpen(false);
  };

  const identity = user?.email ?? user?.name ?? "";

  const loginButton = (
    <div className="flex items-center gap-2 shrink-0">
      <span className="text-xs text-text-secondary max-w-[10rem] leading-tight">
        By signing in you agree to our{" "}
        <Link to="/privacy" className="underline hover:text-text">
          Privacy Policy
        </Link>
      </span>
      <a
        href={`${apiBaseUrl}/login/google`}
        className="inline-flex items-center justify-center px-4 py-2 rounded shrink-0 bg-accent hover:bg-accent-dark text-white font-medium no-underline transition-colors"
      >
        Log in
      </a>
    </div>
  );

  const accountMenu = (
    <div ref={menuRef} className="relative shrink-0">
      <button
        type="button"
        onClick={() => setOpen((v) => !v)}
        aria-haspopup="menu"
        aria-expanded={open}
        aria-label="Account menu"
        className="inline-flex items-center justify-center w-10 h-10 rounded-full bg-surface-light text-text border border-border transition-colors hover:bg-surface"
      >
        <PersonIcon aria-hidden fontSize="1.375rem" />
      </button>

      {open && (
        <div
          role="menu"
          className="absolute right-0 mt-2 w-56 rounded-lg bg-bg border border-border shadow-lg overflow-hidden z-50"
        >
          <div className="px-4 py-3 border-b border-border">
            <p className="text-sm font-semibold text-text truncate" title={identity}>
              {identity}
            </p>
          </div>
            {isAdmin && (
            <Link
              to="/staff"
              role="menuitem"
              onClick={() => setOpen(false)}
              className="flex items-center gap-2 w-full text-left px-4 py-2.5 text-sm font-medium text-text no-underline hover:bg-surface transition-colors"
            >
              <PersonIcon aria-hidden /> Staff Profile
            </Link>
          )}
          <Link
            to="/pageReports"
            role="menuitem"
            onClick={() => setOpen(false)}
            className="flex items-center gap-2 w-full text-left px-4 py-2.5 text-sm font-medium text-text no-underline hover:bg-surface transition-colors"
          >
            <PersonIcon aria-hidden /> Page Reports
          </Link>
          <button
            type="button"
            role="menuitem"
            onClick={handleLogout}
            className="w-full text-left px-4 py-2.5 text-sm font-medium text-text hover:bg-surface transition-colors border-t border-border"
          >
            Log out
          </button>
        </div>
      )}
    </div>
  );

  return (
    <nav
      className="bg-surface-dark border-b border-border px-6 py-2"
      aria-label="Primary"
    >
      <HStack align="center" gap="space-8" wrap={false}>
        <Button
          as={Link}
          to="/"
          variant="tertiary"
          data-color="neutral"
          icon={<HouseIcon aria-hidden />}
        >
          Home
        </Button>

        <Spacer />

        {checked && (user ? accountMenu : loginButton)}
      </HStack>
    </nav>
  );
}

export default Navbar;
