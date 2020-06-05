use arena::*;

lazy_static! {
    static ref VIDEO_FRAME_ARENA: std::sync::Mutex<Arena<std::sync::Arc<std::sync::Mutex<VideoFrame>>>> = {
        std::sync::Mutex::new(Arena::new())
    };
}

impl ArenaAllocator for VideoFrame {
    fn allocate() -> ArenaValueWrapper<VideoFrame> {
        let arena = &mut VIDEO_FRAME_ARENA.lock().unwrap();
        ArenaValueWrapper{   
            index: arena.insert(std::sync::Arc::new(std::sync::Mutex::new(Self::default()))),
            phantom: std::marker::PhantomData
        }
    }

    fn deallocate(index:Index) {
        let arena = &mut VIDEO_FRAME_ARENA.lock().unwrap();
        arena.remove(index);
    }

    fn load(index:Index) -> std::sync::Arc<std::sync::Mutex<Self>> {
        let arena = &VIDEO_FRAME_ARENA.lock().unwrap();
        arena[index].clone()
    }
}

//#[slabify]
#[derive(Default,Debug)]
struct VideoFrame {
}

fn main() {
  let frame = VideoFrame::allocate();
  println!("{:?}",frame.load().lock().unwrap());
}