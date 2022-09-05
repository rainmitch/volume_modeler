use glium::backend::glutin;


extern crate glium;


pub struct Window {
	pub target_fps: u32,
}

impl Window {
	pub fn create_window () -> Window {
		return Window {target_fps: 60};
	}

	pub fn set_target_FPS (&mut self, new_target: u32) {
		self.target_fps = new_target;
	}

	pub fn run_loop<F: FnOnce () + Copy> (&'static self, updateFunc: &'static F) {
		use glium::glutin;

		let mut event_loop = glutin::event_loop::EventLoop::new ();
		let wb = glutin::window::WindowBuilder::new ()
			.with_title ("Volumetric Modeler");
		let cb = glutin::ContextBuilder::new ()
			.with_hardware_acceleration (Some (true))
			.with_vsync (true);
		let display = glium::Display::new (wb, cb, &event_loop).unwrap ();

		
		event_loop.run (move |ev, _, control_flow| {
			let updateStartTime = std::time::Instant::now ();
			updateFunc ();
			let updateDuration = updateStartTime.elapsed ();
			let targetFrameDuration = 1_000_000_000u64 / (self.target_fps as u64);
			let realFrameTime = std::time::Duration::from_nanos (targetFrameDuration) - updateDuration;
			
			//println! ("{}", realFrameTime.as_nanos());

			let next_frame_time = std::time::Instant::now() + realFrameTime;
			*control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);
			match ev {
				glutin::event::Event::WindowEvent { event, .. } => match event {
					glutin::event::WindowEvent::CloseRequested => {
						*control_flow = glutin::event_loop::ControlFlow::Exit;
						return;
					},
					_ => return,
				},
				_ => (),
			}
		});
	}
}