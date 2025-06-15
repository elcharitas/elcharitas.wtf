use momenta::prelude::*;

pub struct CardProps {
    pub children: Vec<Node>,
}

#[component]
pub fn Card(props: &CardProps) -> Node {
    rsx! {
        <div class="overflow-hidden relative duration-300 ease-out transform rounded-lg hover:bg-zinc-400/10 group md:gap-8 shadow-lg hover:shadow-2xl">
            <div class="pointer-events-none">
                <div class="absolute inset-0 z-0 transition duration-1000 [mask-image:linear-gradient(black,transparent)]" />
                <div class="absolute inset-0 z-10 opacity-100 bg-zinc-400/10 transition duration-1000 group-hover:opacity-50" />
                <div class="absolute inset-0 z-10 opacity-0 mix-blend-overlay transition duration-1000 group-hover:opacity-100" />
            </div>
            {&props.children}
        </div>
    }
}
