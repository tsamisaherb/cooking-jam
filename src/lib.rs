use std::f32::consts::PI;

turbo::cfg! {r#"
    name = "Circle Shooter"
    version = "1.0.0"
    author = "Turbo"
    description = "Shoot circles at a target"
    [settings]
    resolution = [300, 300]
"#}

turbo::init! {
    struct GameState {
        angle: f32,
        projectiles: Vec<struct Projectile {
            x: f32,
            y: f32,
            vel_x: f32,
            vel_y: f32,
            radius: f32,
            kind: enum ProjectileKind{Banana = 0, Apple = 1, Pineapple = 2},
        }>,
        targets:Vec<struct Target {
            x: f32,
            y: f32,
            radius: f32,
            vel_x: f32,
            vel_y: f32,
            kind: ProjectileKind
        }>,
        clouds:Vec<struct Cloud {
            x: f32,
            y: f32,
            vel_x: f32
        }>,
        arrow: struct Arrow{
            x: f32,
            y: f32,
            rot: f32,
        },
        //state vars
        score: u32,
        target_timer: f32,
        target_timer_max: f32,
        cloud_timer: f32,
        cloud_timer_max: f32,
        game_timer: f32,
        game_is_over: bool

    } = {
        Self {
            angle: PI/2.0,
            projectiles: Vec::new(),
            targets: Vec::new(),
            clouds: Vec::new(),
            arrow: Arrow{x: 145.0, y: 269.0, rot: 0.0},
            score: 0,
            target_timer: 0.0,
            target_timer_max: 50.0,
            cloud_timer: 0.0,
            cloud_timer_max: 80.0,
            game_timer: 3600.0,
            game_is_over: false
        }
    }
}

const PROJ_KIND : [ProjectileKind; 3] = [ProjectileKind::Apple, ProjectileKind::Banana, ProjectileKind::Pineapple];

turbo::go! {
    let mut state = GameState::load();

    
    // Set the background color
clear(0xADD8E6);

if state.game_timer <= 0.0 && state.projectiles.len() == 0{
    state.game_is_over = true
}

 // Draw the score
 text!(&format!("Score: {}", state.score), x = 10, y = 10, font = Font::L, color = 0xffffffff);
 if state.game_timer == 0.0 && !state.game_is_over{
     text!("LAST SHOT - 2x POINTS", x = 10, y = 30, color = 0x00ff00ff, font = Font::L);
 }
 else if state.game_timer > 0.0{
     text!(&format!("Time Left: {}", (state.game_timer / 60.0 ) as i32), x = 10, y = 30, font = Font::L, color = 0xFF0000);
 }

if !state.game_is_over{
    //make a new target when the game starts
    if state.targets.len() == 0 || state.target_timer <= 0.0{
        let target = Target {
            x: -32.0,
            y: (120 as usize - (rand() as usize %3 * 50 as usize)) as f32,
            vel_x: 1.0,
            vel_y: 0.0,
            radius: 8.0,
            kind: PROJ_KIND[rand() as usize %PROJ_KIND.len() ].clone()
        };
        state.targets.push(target);
        state.target_timer = state.target_timer_max
    }
    if state.clouds.len() == 0 || state.cloud_timer <= 0.0{
        let cloud = Cloud {
            x: -32.0,
            y: (rand() %resolution()[1]) as f32 + 16.0,
            vel_x: 0.25 + (rand() as usize % 100) as f32 / 100.0,
            };
        state.clouds.push(cloud);
        state.cloud_timer = state.cloud_timer_max
    }
 //update the clouds
 state.clouds.retain_mut(|cloud|{
    // Draw the cloud
    sprite!("cloud", x = cloud.x as i32, y = cloud.y as i32);
    cloud.x += cloud.vel_x;
        
    if cloud.x > resolution()[0] as f32{
        false
    }
    else{
        true
    }
});
    //iterate target timer and make a new target if you need one
    state.target_timer = state.target_timer -1.0;
    state.cloud_timer = state.cloud_timer - 1.0;
    state.game_timer -= 1.0;
    if state.game_timer < 0.0 {
        state.game_timer = 0.0
    }
    //make a new projectile when the game starts
    if state.projectiles.len() == 0 {
        //create projectile
        let projectile = Projectile {
            x: 150.0,
            y: 270.0,
            vel_x: 0.0,
            vel_y: 0.0,
            radius: 8.0,
            kind: PROJ_KIND[rand() as usize %PROJ_KIND.len() ].clone()
        };
        state.projectiles.push(projectile);
    }
    // Handle user input
    if gamepad(0).left.pressed() {
        state.angle += 0.05;
    }
    if gamepad(0).right.pressed() {
        state.angle -= 0.05;
    }
    if gamepad(0).start.just_pressed() {
        // give the projectile speed
        let speed = 5.0;
        state.projectiles[0].vel_x = state.angle.cos() * speed;
        state.projectiles[0].vel_y = state.angle.sin() * speed;
    }

    // Update projectiles
    state.projectiles.retain_mut(|projectile| {
        projectile.x += projectile.vel_x;
        projectile.y -= projectile.vel_y;

        // Check for collision with all targets
        let mut hit_target = false;
        let mut score_add = 1;
        state.targets.retain_mut(|target|
        {
            let dx = projectile.x - target.x;
            let dy = projectile.y - target.y;
            let distance = (dx * dx + dy * dy).sqrt();
            let hit = distance < projectile.radius + target.radius 
            && projectile.kind == target.kind;
            if hit {
                if target.y <30.0{
                    score_add = 3
                }
                else if target.y<100.0{
                    score_add = 2
                }
                else{
                    score_add = 1
                }
                hit_target = true
            }
            return !hit
        });

       // if distance < projectile.radius + state.target.radius 
       // && projectile.kind == state.target.kind{
       if hit_target{ 
            if state.game_timer == 0.0{
                score_add = score_add * 2;
            }
            state.score += score_add;
            false // Remove projectile after hitting the target
        } else {
            if projectile.y < 0.0 {
                projectile.vel_y = projectile.vel_y * -1.0
            }
            if projectile.x < 0.0 || projectile.x > resolution()[0] as f32{
                projectile.vel_x = projectile.vel_x * -1.0
            }
           projectile.y < resolution()[1] as f32

        }
    });

    //Update the arrow
    state.arrow.rot = radians_to_degrees(-state.angle) + 90.0;
    //draw the arrow
    sprite!("arrow", x = state.arrow.x as i32, y = state.arrow.y as i32, deg = state.arrow.rot as i32);
    
    //update the target
    state.targets.retain_mut(|target|{
        // Draw the target
        let sprite_name = format!("{:?}", target.kind).to_lowercase();
        sprite!(&sprite_name, x = target.x as i32, y = target.y as i32);
        target.x += target.vel_x;
        target.y += target.vel_y;
            
        if target.x > resolution()[0] as f32{
            false
        }
        else{
            true
        }
    });

   

    // Draw the projectiles
    for projectile in &state.projectiles {
        let sprite_name = format!("{:?}", projectile.kind).to_lowercase();
        sprite!(&sprite_name, x = projectile.x as i32, y = projectile.y as i32);
        }

    
    }
   
    
    // Save game state for the next frame
    state.save();
}

fn radians_to_degrees(radians: f32) -> f32 {
    radians * (180.0 / std::f32::consts::PI)
}