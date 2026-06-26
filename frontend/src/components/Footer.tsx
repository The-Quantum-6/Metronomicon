import { Link as RouterLink } from "react-router-dom";
import { HStack, VStack, Spacer, Heading, BodyShort, Link } from "@navikt/ds-react";

function Footer() {
  return (
    <footer className="bg-surface-dark border-t border-border">
      <div className="w-[min(1200px,84%)] mx-auto py-8">
        <HStack align="start" gap="space-32" wrap>
          <VStack gap="space-1">
            <Heading size="small" level="2">
              Metronomicon
            </Heading>
            <BodyShort textColor="subtle">By students, for students</BodyShort>
            <BodyShort textColor="subtle" className="font-mono">
              Org.nr. 67696769
            </BodyShort>
          </VStack>

          <Spacer />

          <HStack gap="space-24" align="center" wrap>
            <Link as={RouterLink} to="/about" variant="subtle">
              About
            </Link>
            <Link as={RouterLink} to="/privacy" variant="subtle">
              Privacy Policy
            </Link>
            <Link
              href="https://github.com"
              target="_blank"
              rel="noreferrer"
              variant="subtle"
            >
              GitHub
            </Link>
          </HStack>
        </HStack>
      </div>
    </footer>
  );
}

export default Footer;
