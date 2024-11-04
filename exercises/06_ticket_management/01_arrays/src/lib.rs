// TODO: Flesh out the `WeekTemperatures` struct and its method implementations to pass the tests.

pub struct WeekTemperatures {
    // TODO
    week_temperatures:  [Option<i32>; 7],//=[None; 7];
}


pub enum Weekday {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}

use Weekday::*;
impl WeekTemperatures {
    pub fn new() -> Self {
        WeekTemperatures{ week_temperatures: [None;7] }
    }

    pub fn get_temperature(&self, day: Weekday) -> Option<i32> {

        match day
        {
            Monday=>self.week_temperatures[0],
            Tuesday=>self.week_temperatures[1],
            Wednesday=>self.week_temperatures[2],
            Thursday=>self.week_temperatures[3],
            Friday=>self.week_temperatures[4],
            Saturday=>self.week_temperatures[5],
            Sunday=>self.week_temperatures[6],
        }
    }

    pub fn set_temperature(&mut self, day: Weekday, temperature: i32) {
        match day
        {
            Monday=>self.week_temperatures[0]=Some(temperature),
            Tuesday=>self.week_temperatures[1]=Some(temperature),
            Wednesday=>self.week_temperatures[2]=Some(temperature),
            Thursday=>self.week_temperatures[3]=Some(temperature),
            Friday=>self.week_temperatures[4]=Some(temperature),
            Saturday=>self.week_temperatures[5]=Some(temperature),
            Sunday=>self.week_temperatures[6]=Some(temperature),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_temperature() {
        let mut week_temperatures = WeekTemperatures::new();

        assert_eq!(week_temperatures.get_temperature(Weekday::Monday), None);
        assert_eq!(week_temperatures.get_temperature(Weekday::Tuesday), None);
        assert_eq!(week_temperatures.get_temperature(Weekday::Wednesday), None);
        assert_eq!(week_temperatures.get_temperature(Weekday::Thursday), None);
        assert_eq!(week_temperatures.get_temperature(Weekday::Friday), None);
        assert_eq!(week_temperatures.get_temperature(Weekday::Saturday), None);
        assert_eq!(week_temperatures.get_temperature(Weekday::Sunday), None);

        week_temperatures.set_temperature(Weekday::Monday, 20);
        assert_eq!(week_temperatures.get_temperature(Weekday::Monday), Some(20));

        week_temperatures.set_temperature(Weekday::Monday, 25);
        assert_eq!(week_temperatures.get_temperature(Weekday::Monday), Some(25));

        week_temperatures.set_temperature(Weekday::Tuesday, 30);
        week_temperatures.set_temperature(Weekday::Wednesday, 35);
        week_temperatures.set_temperature(Weekday::Thursday, 40);
        week_temperatures.set_temperature(Weekday::Friday, 45);
        week_temperatures.set_temperature(Weekday::Saturday, 50);
        week_temperatures.set_temperature(Weekday::Sunday, 55);

        assert_eq!(week_temperatures.get_temperature(Weekday::Monday), Some(25));
        assert_eq!(
            week_temperatures.get_temperature(Weekday::Tuesday),
            Some(30)
        );
        assert_eq!(
            week_temperatures.get_temperature(Weekday::Wednesday),
            Some(35)
        );
        assert_eq!(
            week_temperatures.get_temperature(Weekday::Thursday),
            Some(40)
        );
        assert_eq!(week_temperatures.get_temperature(Weekday::Friday), Some(45));
        assert_eq!(
            week_temperatures.get_temperature(Weekday::Saturday),
            Some(50)
        );
        assert_eq!(week_temperatures.get_temperature(Weekday::Sunday), Some(55));
    }
}
