use crate::simulation_confetti::SimulationStateConfetti;
use crate::simulation_fireworks::SimulationStateFireworks;
use ratatui::prelude::*;

#[derive(Debug)]
pub enum AppSimulation {
    Fireworks(SimulationStateFireworks),
    Confetti(SimulationStateConfetti),
}

impl StatefulWidget for AppSimulation {
    type State = AppSimulation; // Change the associated type to use the enum

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        match state {
            AppSimulation::Fireworks(state) => {
                let mut indices_to_remove = vec![];
                let mut indices_to_explode = vec![];
                for (index, particle) in state.particles.iter().enumerate() {
                    let pos = particle.physics.position();

                    if pos.x < 0.0 || pos.x >= area.width as f32 || pos.y >= area.height as f32 {
                        indices_to_remove.push(index);
                        continue;
                    } else if particle.shooting && particle.physics.velocity().y > -3.0 {
                        indices_to_explode.push(index);
                        indices_to_remove.push(index);
                        continue;
                    }

                    if pos.y.floor() > -1.0 {
                        let cell = buf.get_mut(pos.x.floor() as u16, pos.y.floor() as u16);
                        cell.set_char(particle.char); // Set the character
                        cell.fg = particle.color;
                    }

                    if particle.shooting {
                        let l = -particle.physics.velocity().y as isize;
                        for i in 1..l {
                            let y = pos.y as isize + i;
                            if y > 0 && y < (area.height - 1) as isize {
                                let cell = buf.get_mut(pos.x.floor() as u16, y as u16);
                                cell.set_char(particle.tail_char.unwrap()); // Set the character
                                cell.fg = particle.color;
                            }
                        }
                    }
                }

                for &index in indices_to_explode.iter().rev() {
                    let color = state.particles[index].color;
                    let pos = &state.particles[index].physics.position();
                    let x = pos.x;
                    let y = pos.y;
                    state.spawn_explosion_particles(color, x, y);
                }

                state.remove_indices_from_particles(indices_to_remove);
            }
            AppSimulation::Confetti(state) => {
                let mut indices_to_remove = vec![];
                for (index, particle) in state.particles.iter().enumerate() {
                    let pos = particle.physics.position();

                    if pos.x < 0.0
                        || pos.x >= area.width as f32
                        || pos.y < 0.0
                        || pos.y >= area.height as f32
                    {
                        indices_to_remove.push(index);
                        continue;
                    }

                    let cell = buf.get_mut(pos.x.floor() as u16, pos.y.floor() as u16);
                    cell.set_char(particle.char); // Set the character
                    cell.fg = particle.color;
                }

                state.remove_indices_from_particles(indices_to_remove);
            }
        }
    }
}
