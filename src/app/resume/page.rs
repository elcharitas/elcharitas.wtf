use crate::components::PageLayout;
use crate::shared::PageParams;
use momenta::prelude::*;

#[component]
pub fn ResumePage(_: &PageParams) -> Node {
    rsx! {
        <PageLayout title="Resume">
            <div class="max-w-4xl mx-auto px-4 text-white">
                <iframe class="min-h-screen w-full" src="/resume.docx" />
            </div>
        </PageLayout>
    }
}
