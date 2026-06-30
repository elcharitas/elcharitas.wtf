use momenta::prelude::*;

pub struct CardProps {
    pub children: Vec<Node>,
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
