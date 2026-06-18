
import { Button, HStack } from "@navikt/ds-react";

function FeideButton() {
  return (
    <>  <HStack gap="space-8">
            <Button as="a" variant="primary" data-color="accent" href="http://localhost:3000/login/feide">Login med FEIDE</Button>
        </HStack>
    </>
  )
}

export default FeideButton
