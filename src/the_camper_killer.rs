/*
*   Auteurs : "HYSENI Riad", "EL HAKOUNI Yassin" & El Magnifico "ELMI Navid"
*   Notre vaisseau fighter : THE CAMPER KILLER
*   DISCLAIMER: On n'utilise pas la radio par stratégie afin de ne pas se faire brouiller nos missiles
*               A la place on utilise les radars sur les missiles
*/

use oort_api::prelude::*;
 
const MIN_ANGLE_DIFF: f64 = 0.005;             // Tolérance d'alignement avant de tirer
const DECELERATE_DISTANCE: f64 = 5000.0;       // Distance à partir de laquelle on doit ralentir
const MAX_SPEED: f64 = 300.0;         // Vitesse maximale commune au fighter et aux missiles
const SHIP_TURN_SPEED: f64 = 2.0 * std::f64::consts::PI; // Vitesse de rotation du vaisseau donnée par l'API
const EXPLOSION_RANGE: f64 = 200.0; // Distance minimale avant que les missiles n'explosent (mise en constante pour tests)
const BULLET_SPEED: f64 = 1000.0; // Vitesse des balles donnée par l'API
 
pub struct Ship {}
 
impl Ship {
    pub fn new() -> Ship {
        Ship {}
    }
 
    fn adjust_ship_speed(&mut self, enemy_position: Vec2) {
        let direction_to_enemy = enemy_position - position();
        let distance_to_enemy = direction_to_enemy.length();
        let target_speed: f64 = if distance_to_enemy < DECELERATE_DISTANCE {
            (distance_to_enemy / DECELERATE_DISTANCE) * MAX_SPEED
        } else {
            MAX_SPEED
        };
 
        let current_speed = velocity().length();
        if current_speed > target_speed {
            accelerate(velocity().normalize() * -max_forward_acceleration()); // Ralentir si trop rapide
        } else if current_speed < target_speed {
            accelerate(direction_to_enemy.normalize() * max_forward_acceleration()); // Accélérer vers l'ennemi (comme des bonhommes)
        }
    }
 
    pub fn tick(&mut self) {
        if class() == Class::Fighter {
            self.tick_fighter();
        } else if class() == Class::Missile {
            self.tick_missile();
        }
    }
    
    fn predict_target_position(&mut self, enemy_position: Vec2, enemy_velocity: Vec2) -> Vec2 {
        //let mut time_to_target = (predicted_position - position()).length() / BULLET_SPEED; formule (fonctionnelle) utilisée pour l'exo 5
 
        let future_position = enemy_position + enemy_velocity * 0.005;
 
        let bullet_time_to_target = (future_position - position()).length() / BULLET_SPEED;
        
        future_position + enemy_velocity * bullet_time_to_target * 0.005 // Ajustement avec une constante qui va bien
    }
 
    pub fn tick_fighter(&mut self) {
        set_radar_width(0.1); //range ideale pour le fighter duel
        if let Some(contact) = scan() {
            let enemy_position = contact.position;
            let enemy_velocity = contact.velocity;

            let future_position = self.predict_target_position(enemy_position, enemy_velocity);
            self.adjust_ship_speed(enemy_position);
 
            // Calculer la direction vers la position future de l'ennemi
            let direction_to_future_position = future_position - position();
            let angle_to_future_position = direction_to_future_position.angle();
            let angle_difference_future = angle_diff(heading(), angle_to_future_position);
 
            if angle_difference_future.abs() > MIN_ANGLE_DIFF {
                let turn_amount = SHIP_TURN_SPEED * angle_difference_future / std::f64::consts::PI;
                if turn_amount.abs() > MIN_ANGLE_DIFF {
                    turn(turn_amount); 
                }
            }
 
            // Tirer en continue pour mitrailler l'ennemi 
            fire(0);
            fire(1); 
 
            draw_line(position(), enemy_position, 0x00ff00);  // Ligne vers l'ennemi
            draw_line(position(), future_position, 0xff0000); // Ligne vers la position future de l'ennemi
            debug!("Distance to enemy: {}", direction_to_future_position.length());
 
        } else {
            set_radar_heading(radar_heading() + 0.1);
        }
    }
 
    pub fn tick_missile(&mut self) {
        self.missile_adjust_traj();
        // Si le missile est assez proche de la cible, faire exploser le missile
        if let Some(contact) = scan() {
            let distance_to_target = (contact.position - position()).length();
            if distance_to_target < EXPLOSION_RANGE { 
                explode(); 
            }
        }
    }
 
    pub fn missile_adjust_traj(&mut self) 
    {   
        set_radar_width(0.05); //range ideale pour le fighter duel
        let scan_result = scan();
        if let Some(result) = scan_result {
            let my_position = position();         
            let enemy_position = result.position; 
            let enemy_velocity = result.velocity; 
 
            let future_position = self.predict_target_position(enemy_position, enemy_velocity);
 
            // Calculer la direction vers la position future de l'ennemi
            let direction_to_target = future_position - my_position;
            let angle_to_future_target = direction_to_target.angle(); 
            let angle_difference = angle_diff(heading(), angle_to_future_target);
 
            if angle_difference.abs() > MIN_ANGLE_DIFF {
                let turn_amount = 30.0 * angle_difference / std::f64::consts::PI;
                turn(turn_amount); 
            }
            set_radar_heading(angle_to_future_target);
            //foncer vers l'ennemi
            accelerate(direction_to_target.normalize() * MAX_SPEED);
        } else {
            // Si aucun ennemi n'est détecté, scanner avec le radar
            set_radar_heading(radar_heading() + 0.1);
        }
    }
}
