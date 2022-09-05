

mod window;

pub fn test () {

}

static win: window::Window = window::Window {
	target_fps: 60,
};

fn main () {
	win.run_loop (&|| -> () {
		
	});
}
