use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(PeoplePlugin)
        .run();
}

pub struct PeoplePlugin;

impl Plugin for PeoplePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup)
            .add_system(print_names)
            .add_system(people_with_jobs)
            .add_system(people_without_jobs)
            .add_system(person_does_job);
    }
}

pub fn setup(mut commands: Commands) {
    commands.spawn(
        (Person {
            name: "Jurgen".into()
        },
        Employed {
            job: Job::Doctor,
        })
    );
    commands.spawn(
        (Person {
            name: "Charles".into()
        },
        Employed {
            job: Job::FireFighter,
        })
    );
    commands.spawn(
        (Person {
            name: "Erick".into()
        },)
    );
}

pub fn print_names(person_query: Query<&Person>) {
    for person in person_query.iter() {
        println!("Name: {}", person.name)
    }
}

pub fn people_with_jobs(person_query: Query<&Person, With<Employed>>) {
    for person in person_query.iter() {
        println!("{} has a job", person.name)
    }
}

pub fn people_without_jobs(person_query: Query<&Person, Without<Employed>>) {
    for person in person_query.iter() {
        println!("{} is ready for hire!", person.name)
    }
}

pub fn person_does_job(person_query: Query<(&Person, &Employed)>) {
    for (person, employed) in person_query.iter() {
        let job_name = match employed.job {
            Job::Doctor => "Doctor",
            Job::FireFighter => "Firefighter",
            Job::Lawyer => "Lawyer"
        };
        println!("{} is a {}", person.name, job_name)
    }
}

#[derive(Component)]
pub struct Person {
    pub name: String,

}

#[derive(Component)]
pub struct Employed {
    pub job: Job
}

#[derive(Debug)]
pub enum Job {
    Doctor,
    FireFighter,
    Lawyer
}