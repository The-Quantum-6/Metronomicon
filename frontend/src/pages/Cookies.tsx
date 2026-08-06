import Navbar from "../components/Navbar";
import Footer from "../components/Footer";

export default function Cookies() {
  return (
    <div className="min-h-screen flex flex-col bg-surface-dark text-text">
      <Navbar />
      <main className="container mx-auto max-w-3xl p-6 flex-1">
        <h1 className="text-2xl font-bold text-primary mb-1">Cookie Policy</h1>
        <p className="text-sm text-text-muted mb-6">Last updated: [DATE]</p>

        <p className="text-text-secondary mb-4">
          Metronomicon only uses cookies that are strictly necessary to sign you in and keep the
          site secure. We do not use cookies for analytics, advertising, or tracking.
        </p>

        <ul className="list-disc pl-5 text-text-secondary space-y-2 mb-4">
          <li>A login token that keeps you signed in after you log in with Google.</li>
          <li>A session cookie used during sign-in to keep your login secure.</li>
          <li>Permission tokens that let the site show the right actions for each course.</li>
        </ul>

        <p className="text-text-secondary mb-4">
          When you log out, your login cookie is cleared.
        </p>

        <p className="text-text-secondary">
          Signing in redirects you to Google, which may set its own cookies. That is governed by{" "}
          <a
            href="https://policies.google.com/privacy"
            target="_blank"
            rel="noreferrer"
            className="text-accent hover:underline"
          >
            Google's Privacy Policy
          </a>
          .
        </p>
      </main>
      <Footer />
    </div>
  );
}
