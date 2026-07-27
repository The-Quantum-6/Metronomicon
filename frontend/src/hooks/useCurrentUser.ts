import { useEffect, useState } from "react";
import { apiUrl } from "../config";

export type CurrentUser = {
  sub: string;
  role: "user" | "admin" | "root";
};

export function useCurrentUser() {
  const [me, setMe] = useState<CurrentUser | null>(null);

  useEffect(() => {
    let cancelled = false;
    (async () => {
      try {
        const response = await fetch(apiUrl("me"), { credentials: "include" });
        // Missing/expired token redirects to the login flow, so only trust an ok JSON response
        if (response.ok && !cancelled) {
          setMe(await response.json());
        }
      } catch {
        // Not logged in — leave me as null
      }
    })();
    return () => {
      cancelled = true;
    };
  }, []);

  const isStaff = me?.role === "admin" || me?.role === "root";
  return { me, isStaff };
}
