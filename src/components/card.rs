use momenta::prelude::*;

pub struct CardProps {
    pub children: Vec<Node>,
}

#[component]
pub fn Card(props: &CardProps) -> Node {
    rsx! {
        <div class={"overflow-hidden relative duration-300 ease-out transform rounded-lg hover:bg-zinc-400/10 group md:gap-8 shadow-lg hover:shadow-2xl"}>
            <div class="pointer-events-none">
                <div class="absolute inset-0 z-0 transition duration-1000" />
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
        <div data_show="$has_next_page" class={"mt-12"} data_on_intersect={intersect}>
            <div class="mx-auto max-w-md rounded-xl border border-zinc-800 bg-zinc-950/60 px-4 py-3">
                <div class="flex items-center justify-center gap-3 text-zinc-500">
                    <span class="inline-block w-2 h-2 rounded-full bg-zinc-500 animate-pulse"></span>
                    <span class="text-xs uppercase tracking-[0.1em] font-semibold">"Loading more entries"</span>
                    <span class="inline-block w-2 h-2 rounded-full bg-zinc-500 animate-pulse"></span>
                </div>
            </div>
        </div>
    }
}
