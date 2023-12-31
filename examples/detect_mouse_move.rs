use inputbot::{BlockInput::*, KeybdKey::*, MouseButton, MouseCursor};

// This example demonstrates capturing mouse movement and either allowing it to proceed
// or blocking for some reason, such as sending a custom mouse position

fn main() {

    static mut VERTICAL_ONLY:bool = false;
    static mut HORIZONTAL_ONLY:bool = false;
    static mut FROZEN_X_VAL:i32 = 0;
    static mut FROZEN_Y_VAL:i32 = 0;



    MouseButton::MouseMove.blockable_bind(||{

        let mut pos = MouseCursor::pos();
        //println!("{x},{y}", x = pos.0,y=pos.1);

        unsafe {
        
            if F2Key.is_pressed()&&!HORIZONTAL_ONLY{
                FROZEN_Y_VAL = pos.1;
                HORIZONTAL_ONLY = true;
                println!("Enabled, x: {}, y: {}",pos.0,pos.1);
                MouseCursor::move_abs(pos.0,pos.1);
                return Block;
            }
            else if F2Key.is_pressed() && HORIZONTAL_ONLY
            {
                pos.1 = FROZEN_Y_VAL;
                println!("Locked, x: {}, y: {}",pos.0,pos.1 );
                MouseCursor::move_abs(pos.0,pos.1);
                return Block;
            }
            else if !F2Key.is_pressed() && HORIZONTAL_ONLY{
                HORIZONTAL_ONLY = false;
                println!("Released horizontal.");
            }

            if F3Key.is_pressed()&&!VERTICAL_ONLY{
                FROZEN_X_VAL = pos.0;
                VERTICAL_ONLY = true;
                //println!("Enabled vertical only, y: {}",FROZEN_X_VAL);
            }
            else if F3Key.is_pressed() && VERTICAL_ONLY
            {
                pos.0 = FROZEN_X_VAL;
                //println!("Locked to horizontal, y: {}", FROZEN_Y_VAL);
            }
            else if !F3Key.is_pressed() && VERTICAL_ONLY{
                VERTICAL_ONLY = false;
                //println!("Released horizontal.");
            }
        }

        
        DontBlock
        //Block
    });

    // Call this to start listening for bound inputs.
    inputbot::handle_input_events();
}
