import { Link } from "react-router-dom";
import { HStack, Spacer, Button } from "@navikt/ds-react";
import { HouseIcon, PersonIcon } from "@navikt/aksel-icons";

function Navbar() {
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

        <Link
          to="/login"
          aria-label="Profil"
          className="inline-flex items-center justify-center w-10 h-10 rounded-full shrink-0 bg-surface-light text-text border border-border no-underline transition-colors hover:bg-surface"
        >
          <PersonIcon aria-hidden fontSize="1.375rem" />
        </Link>
      </HStack>
    </nav>
  );
}

export default Navbar;
