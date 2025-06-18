use momenta::prelude::*;

pub struct CardProps {
    pub children: Vec<Node>,
}

#[component]
pub fn Card(props: &CardProps) -> Node {
    rsx! {
        <div class={"overflow-hidden relative duration-300 ease-out transform rounded-lg hover:bg-zinc-400/10 group md:gap-8 shadow-lg hover:shadow-2xl"}>
            <div class="pointer-events-none">
                <div class="absolute inset-0 z-0 transition duration-1000 [mask-image:linear-gradient(black,transparent)]" />
                <div class="absolute inset-0 z-10 opacity-100 bg-zinc-400/10 transition duration-1000 group-hover:opacity-50" />
                <div class="absolute inset-0 z-10 opacity-0 mix-blend-overlay transition duration-1000 group-hover:opacity-100" />
            </div>
            {&props.children}
        </div>
    }
}

pub struct ScrollCardProps {
    pub intersect: String,
}

#[component]
pub fn ScrollCard(ScrollCardProps { intersect }: &ScrollCardProps) -> Node {
    rsx! {
        // Scroll indicator
        <div data_show="$has_next_page" class={"animate-bounce mt-12"} data_on_intersect={intersect}>
            <div class="flex flex-col items-center gap-2 text-zinc-500">
                <span class="text-xs font-medium">"Scroll to explore"</span>
                <i class="fas fa-chevron-down text-sm"></i>
            </div>
        </div>
    }
}
