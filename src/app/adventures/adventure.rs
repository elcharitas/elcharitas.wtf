use crate::{derive_component, derive_props, shared::*};
use hypertext::rsx_move;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct YearData {
    month: String,
    info: Vec<String>,
}

derive_props! {
    AdventureProps {
        year: String,
        data: Vec<YearData>,
    }
}

derive_component! {
    pub Adventure(props: AdventureProps) {
        let AdventureProps { year, data } = props;
        rsx_move! {
            <li class="mb-2 flex flex-col items-center">
                <div class="flex flex-col md:w-3/4">
                <div
                    tabindex=0
                    class="text-zinc-500 font-display animate-title text-center text-4xl my-8 py-4 duration-700 hover:text-yellow-500/80 hover:scale-110"
                >
                    {year}
                </div>
                {data.iter().enumerate().map(|(pos, YearData { month, info })| (
                    rsx_move! {
                        <div class=format!("`mb-4 md:ml-4 md:mt-8 animate-fade-in md:flex flex-col {}", if (pos % 2) == 0 { "items-start" } else { "items-end" })>
                        <div>
                            <div class="p-4 max-w-[500px] min-w-[300px] md:w-[500px]">
                            <div class="px-4 pb-4 w-full">
                                <h3 class="text-lg font-display uppercase my-2 text-yellow-500/80">
                                {month}
                                </h3>
                                {info.iter().map(|_data| (
                                    rsx_move! {
                                        <p class="text-zinc-400">
                                            <span class="text-yellow-500">&#8226;</span> &nbsp;{" "}
                                            // <span text={&data} />
                                        </p>
                                    }
                                )).render_all()}
                            </div>
                            </div>
                        </div>
                        </div>
                    }
                )).render_all()}
                </div>
            </li>
        }
    }
}
