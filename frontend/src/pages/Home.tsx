import Navbar from "../components/Navbar";
import Footer from "../components/Footer";
import { useEffect, useState } from "react";
import { Link } from "react-router-dom";
import {
  Box,
  Page,
  Heading,
  BodyShort,
  Detail,
  HGrid,
  VStack,
  LinkCard,
  Loader,
  Theme,
} from "@navikt/ds-react";
import { apiUrl } from "../config";

type Course = {
  id: string;
  name: string;
  code: string;
};

function Home() {
  const [courses, setCourses] = useState<Course[] | null>(null);

  useEffect(() => {
    fetch(apiUrl("courses"))
      .then((r) => {
        if (!r.ok) throw new Error(`Failed to load courses (${r.status})`);
        return r.json() as Promise<Course[]>;
      })
      .then((data) => setCourses(data))
      .catch(() => setCourses([]));
  }, []);

  return (
    <Theme>
      <Box background="default" className="min-h-screen flex flex-col">
        <Navbar />
        <Page.Block as="main" width="xl" gutters className="w-[min(1200px,84%)] mx-auto flex-1">
          <VStack gap="space-40" paddingBlock="space-64 space-80">
            <Heading size="xlarge" level="1" className="text-[clamp(2.5rem,6vw,5rem)] leading-[1.05] text-center">
              Welcome to Metronomicon
            </Heading>

            <section>
              <Heading size="large" level="2" spacing>
                Courses
              </Heading>

              {courses === null ? (
                <Loader size="large" title="Loading courses" />
              ) : courses.length === 0 ? (
                <BodyShort textColor="subtle">No courses yet.</BodyShort>
              ) : (
                <HGrid gap="space-24" columns={{ xs: 1, sm: 2, lg: 3 }}>
                  {courses.map((c) => (
                    <LinkCard key={c.id} data-color="neutral" className="course-card">
                      <LinkCard.Title as="h3">
                        <Detail
                          as="div"
                          className="uppercase tracking-[0.06em] text-text-secondary mb-1"
                        >
                          {c.code}
                        </Detail>
                        <LinkCard.Anchor asChild>
                          <Link to={`/courses/${c.id}`}>{c.name}</Link>
                        </LinkCard.Anchor>
                      </LinkCard.Title>
                    </LinkCard>
                  ))}
                </HGrid>
              )}
            </section>
          </VStack>
        </Page.Block>
        <Footer />
      </Box>
    </Theme>
  );
}

export default Home;
