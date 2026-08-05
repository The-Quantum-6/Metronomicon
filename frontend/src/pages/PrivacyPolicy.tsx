import Navbar from "../components/Navbar";
import Footer from "../components/Footer";

export default function PrivacyPolicy() {
  return (
    <div className="min-h-screen flex flex-col bg-surface-dark text-text">
      <Navbar />
      <main className="container mx-auto max-w-3xl p-6 flex-1">
        <h1 className="text-2xl font-bold text-primary mb-1">Privacy Policy</h1>
        <p className="text-sm text-text-muted mb-6">Last updated: [05.08.2026]</p>

        <p className="text-text-secondary mb-4">
          Metronomicon is a student-built platform for sharing course information at OsloMet.
          This policy explains what personal data we process, why, and the rights
          you have under the GDPR. For any privacy question or request, please send in a <span className="text-text">report </span> 
          at our main page with contact information for a follow-up.
        </p>

        <h2 className="text-lg font-semibold text-primary mt-6 mb-2">The data we process</h2>
        <ul className="list-disc pl-5 text-text-secondary space-y-1 mb-4">
          <li>
            <b>Account data.</b> You sign in with Google (via OpenID Connect). We receive a unique
            account identifier, your name, and your email address. We never receive your password.
          </li>
          <li>
            <b>Permissions and roles.</b> The permissions and role assigned to your account, so we
            can control who may suggest, moderate, or publish content.
          </li>
          <li>
            <b>Content you submit.</b> Links, resources and uploaded files, project ideas, FAQ
            entries, and reports, along with the fact that your account submitted them. If you add
            an optional contact email to a report, we store that too.
          </li>
          <li>
            <b>Technical data.</b> Authentication tokens and cookies needed to keep you signed in
            and enforce permissions, and standard operational data required to run and secure the
            service.
          </li>
        </ul>
        <p className="text-text-secondary mb-4">
          We do not use analytics, advertising, or third-party tracking.
        </p>

        <h2 className="text-lg font-semibold text-primary mt-6 mb-2">Why we process it</h2>
        <ul className="list-disc pl-5 text-text-secondary space-y-1 mb-4">
          <li>To authenticate you and secure accounts.</li>
          <li>To operate the contribution and moderation system and enforce permissions.</li>
          <li>To display content you choose to publish.</li>
        </ul>

        <h2 className="text-lg font-semibold text-primary mt-6 mb-2">Storage and deletion</h2>
        <p className="text-text-secondary mb-4">
          Content is stored as an append-only history so changes can be reviewed and earlier
          versions restored, which means it is not erased automatically. To have personal or
          unlawful information removed, send us a report from the main page; we remove it from what
          is shown and are building tooling for full erasure.
        </p>

        <h2 className="text-lg font-semibold text-primary mt-6 mb-2">Sharing and retention</h2>
        <p className="text-text-secondary mb-4">
          We do not sell your data. We share it only with the providers needed to run the service —
          Google for login, and our hosting and storage providers (including Amazon Web Services) —
          and with authorities where legally required. Content you publish is visible to other
          users, and account data is kept for as long as your account exists.
        </p>

        <h2 className="text-lg font-semibold text-primary mt-6 mb-2">Your rights</h2>
        <p className="text-text-secondary">
          Under the GDPR you can access, correct, or delete your data, and object to or restrict its
          use. To make a request, send us a report from the main page. You may also complain to the
          Norwegian Data Protection Authority (Datatilsynet).
        </p>
      </main>
      <Footer />
    </div>
  );
}
