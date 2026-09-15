use momenta::prelude::*;

pub struct ScrollCardProps {
    pub intersect: String,
}

#[component]
pub fn ScrollCard(ScrollCardProps { intersect }: &ScrollCardProps) -> Node {
    rsx! {
        <div data_show="$has_next_page" class={"mt-4 border-t border-zinc-900/15 py-6"} data_on_intersect={intersect}>
            <p class="text-center text-xs uppercase tracking-[0.12em] text-zinc-500">"Loading more entries…"</p>
        </div>
    }
}
