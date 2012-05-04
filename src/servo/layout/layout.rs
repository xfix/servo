#[doc = "

The layout task. Performs layout on the dom, builds display lists and sends
them to be rendered

"];

import task::*;
import comm::*;
import gfx::geom;
import gfx::geom::*;
import gfx::renderer;
import dom::base::*;
import display_list::*;
import dom::rcu::scope;

enum msg {
    build,
    exit
}

fn layout(renderer: chan<renderer::msg>) -> chan<msg> {

    spawn_listener::<msg> {|po|

        let s = scope();
        let dom = s.new_node(nk_div);

        loop {
            alt recv(po) {
              build {

                let box = layout_dom(dom);
                let dlist = build_display_list(box);

                send(renderer, gfx::renderer::render(dlist));
              }
              exit {
                break;
              }
            }
        }
    }

}

fn layout_dom(dom: node) -> @base::box {
    base::new_box(dom)
}

fn build_display_list(_box: @base::box) -> display_list::display_list {
    let r = core::rand::rng();
    [
        display_item({
            item_type: solid_color,
            bounds: geom::box(
                int_to_au(r.next() as int % 800 - 100),
                int_to_au(r.next() as int % 600 - 100),
                int_to_au(200),
                int_to_au(200))
        }),
        display_item({
            item_type: solid_color,
            bounds: geom::box(
                int_to_au(r.next() as int % 800 - 100),
                int_to_au(r.next() as int % 600 - 100),
                int_to_au(200),
                int_to_au(200))
        })
    ]
}