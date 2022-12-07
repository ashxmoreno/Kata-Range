//Regex - libreria para validar patrones.
use regex::Regex; // 1.7.0

//enum para los errores posibles dentro de los rangos
//derive -- agregar funcionalidad (debug - imprimir estructuras)
#[derive(Debug)]
enum RangeError {
    InvalidArgument
}

#[derive(Debug)]
// struct permite definir propiedades
struct Range {
    pub start_open: bool,
    pub start_value: i32,
    pub end_value: i32,
    pub end_open: bool
}

//impl permite definir métodos
impl Range {
    pub fn new(range: String) -> Result<Range, RangeError> {
        let re = Regex::new(r"(\(|\[)\d+,\d+(\)|\])").unwrap();
        
        if !re.is_match(&range) {
            return Err(RangeError::InvalidArgument)
        }
        
        let range_len = range.len();
        let comma_index = range.find(",").unwrap();
        
        //primer valor (open or closed)
        let start_open = range.chars().nth(0).unwrap() == '(';
        //primer num
        let start_value: i32 = range[1..(comma_index)].parse().unwrap();
        //ultimo num
        let end_value: i32 = range[(comma_index + 1)..(range_len - 1)].parse().unwrap();
        //ultimo valor (open or closed)
        let end_open = range.chars().nth(range_len - 1).unwrap() == ')';
        
        //Si el primer valor (num) es mayor o igual que el ultimo valor (num), lanzar error
        if start_value >= end_value {
            return Err(RangeError::InvalidArgument)
        }
        //si todo va bien, devuelveme el rango con los valores
        Ok(Range { start_open, start_value, end_value, end_open })
    }
    
    
    //self - "este valor"
    pub fn contains(&self, mut integer_range: Vec<i32>) -> bool {
        integer_range.sort();
        
        let is_open_valid = (integer_range[0] >= self.start_value && !self.start_open) || (integer_range[0] > self.start_value && self.start_open);
        let is_end_valid = (integer_range[integer_range.len() - 1] <= self.end_value && !self.end_open) || (integer_range[integer_range.len() - 1] < self.end_value && self.end_open);
        
        //Si ambas validaciones son validad = true, en caso de que una sea falsa o las dos = false.
        //si no hay punto y coma, no hay que poner return. 
        is_open_valid && is_end_valid
    }
    
    pub fn all_points(&self) -> Vec<i32> {
        let start_point = if self.start_open { self.start_value + 1 } else { self.start_value };
        let end_point = if self.end_open { self.end_value } else { self.end_value + 1 };
        
        //.collect lo transforma en un arreglo. 
        (start_point..end_point).collect()
    }
    
    //rango contiene al otro rango?
    pub fn contains_range(&self, other: Range) -> bool {
        
        self.contains(other.all_points())
    }
    
    pub fn end_points(&self) -> Vec<i32> {
        
        let points = self.all_points();
        [points[0], points[points.len() - 1]].into_iter().collect()
    }
    
    pub fn overlaps_range(&self, other: Range) -> bool {
        
        self.start_value <= other.end_value && self.end_value >= other.start_value
    }
    
    pub fn equals(&self, other: Range) -> bool {
        
        self.start_value == other.start_value
        && self.end_value == other.end_value
        && self.start_open == other.start_open
        && self.end_open == other.end_open
    }
}

#[cfg(test)]
//modulo de pruebas
mod tests {
    //llamar la estructura del rango y poder usarla en las pruebas
    use crate::Range;

    #[test]
    fn contains_1() {
        let range = Range::new( String::from("[2,6)") ).unwrap();
        let mut integer_range = vec![2,4];
        
        //assert_eq! = Compara los valores que sean iguales. 
        assert_eq!(range.contains(integer_range), true);
    }
    
    #[test]
    fn contains_2() {
        let range = Range::new( String::from("[2,6)") ).unwrap();
        let mut integer_range = vec![-1,1,6,10];
        
        assert_eq!(range.contains(integer_range), false);
    }
    
    #[test]
    fn get_all_points() {
        let range = Range::new( String::from("[2,6)") ).unwrap();
        
        assert_eq!(range.all_points(), vec![2, 3, 4, 5]);
    }
    
    #[test]
    fn does_not_contains_range_1() {
        let range = Range::new( String::from("[2,5)") ).unwrap();
        let other = Range::new( String::from("[7,10)") ).unwrap();
        
        assert_eq!(range.contains_range(other), false);
    }
    
    #[test]
    fn does_not_contains_range_2() {
        let range = Range::new( String::from("[2,5)") ).unwrap();
        let other = Range::new( String::from("[3,10)") ).unwrap();
        
        assert_eq!(range.contains_range(other), false);
    }
    
    #[test]
    fn does_not_contains_range_3() {
        let range = Range::new( String::from("[2,5)") ).unwrap();
        let other = Range::new( String::from("[2,10)") ).unwrap();
        
        assert_eq!(range.contains_range(other), false);
    }
    
    #[test]
    fn contains_range_1() {
        let range = Range::new( String::from("[2,10)") ).unwrap();
        let other = Range::new( String::from("[3,5]") ).unwrap();
        
        assert_eq!(range.contains_range(other), true);
    }
    
    #[test]
    fn contains_range_2() {
        let range = Range::new( String::from("[3,5]") ).unwrap();
        let other = Range::new( String::from("[3,5)") ).unwrap();
        
        assert_eq!(range.contains_range(other), true);
    }
    
    #[test]
    fn end_points_1() {
        let range = Range::new( String::from("[2,6)") ).unwrap();
        
        assert_eq!(range.end_points(), vec![2, 5]);
    }

    #[test]
    fn end_points_2() {
        let range = Range::new( String::from("[2,6]") ).unwrap();
        
        assert_eq!(range.end_points(), vec![2, 6]);
    }

    #[test]
    fn end_points_3() {
        let range = Range::new( String::from("(2,6)") ).unwrap();
        
        assert_eq!(range.end_points(), vec![3, 5]);
    }

    #[test]
    fn end_points_4() {
        let range = Range::new( String::from("(2,6]") ).unwrap();
        
        assert_eq!(range.end_points(), vec![3, 6]);
    }
    
    #[test]
    fn does_not_overlap() {
        let range = Range::new( String::from("[2,5)") ).unwrap();
        let other = Range::new( String::from("[7,10)") ).unwrap();
        
        assert_eq!(range.overlaps_range(other), false);
    }

    #[test]
    fn overlap_1() {
        let range = Range::new( String::from("[2,10)") ).unwrap();
        let other = Range::new( String::from("[3,5)") ).unwrap();
        
        assert_eq!(range.overlaps_range(other), true);
    }

    #[test]
    fn overlap_2() {
        let range = Range::new( String::from("[3,5)") ).unwrap();
        let other = Range::new( String::from("[3,5)") ).unwrap();
        
        assert_eq!(range.overlaps_range(other), true);
    }

    #[test]
    fn overlap_3() {
        let range = Range::new( String::from("[2,5)") ).unwrap();
        let other = Range::new( String::from("[3,10)") ).unwrap();
        
        assert_eq!(range.overlaps_range(other), true);
    }

    #[test]
    fn overlap_4() {
        let range = Range::new( String::from("[3,5)") ).unwrap();
        let other = Range::new( String::from("[2,10)") ).unwrap();
        
        assert_eq!(range.overlaps_range(other), true);
    }
    
    #[test]
    fn equals() {
        let range = Range::new( String::from("[3,5)") ).unwrap();
        let other = Range::new( String::from("[3,5)") ).unwrap();
        
        assert_eq!(range.equals(other), true);
    }

    #[test]
    fn neq_1() {
        let range = Range::new( String::from("[2,10)") ).unwrap();
        let other = Range::new( String::from("[3,5)") ).unwrap();
        
        assert_eq!(range.equals(other), false);
    }

    #[test]
    fn neq_2() {
        let range = Range::new( String::from("[2,5)") ).unwrap();
        let other = Range::new( String::from("[3,10)") ).unwrap();
        
        assert_eq!(range.equals(other), false);
    }

    #[test]
    fn neq_3() {
        let range = Range::new( String::from("[3,5)") ).unwrap();
        let other = Range::new( String::from("[2,10)") ).unwrap();
        
        assert_eq!(range.equals(other), false);
    }
}
