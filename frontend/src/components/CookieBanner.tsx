import { useState } from "react";
import { Link } from "react-router-dom";

const COOKIE_NOTICE_SEEN = "metronomicon_cookie_notice_seen";

export default function CookieBanner() {
  const [dismissed, setDismissed] = useState(
    () => localStorage.getItem(COOKIE_NOTICE_SEEN) === "true"
  );

  if (dismissed) return null;

  const accept = () => {
    localStorage.setItem(COOKIE_NOTICE_SEEN, "true");
    setDismissed(true);
  };

  return (
    <div className="fixed bottom-0 inset-x-0 z-50 bg-bg border-t border-border p-4">
      <div className="container mx-auto max-w-4xl flex flex-col sm:flex-row sm:items-center gap-3">
        <p className="text-sm text-text-secondary flex-1">
          Metronomicon uses only the cookies needed to sign you in and keep the site secure — no
          tracking.{" "}
          <Link to="/cookies" className="text-accent hover:underline">
            Learn more
          </Link>
        </p>
        <button
          onClick={accept}
          className="bg-accent hover:bg-accent-dark text-white px-4 py-2 rounded text-sm font-medium shrink-0"
        >
          OK
        </button>
      </div>
    </div>
  );
}
