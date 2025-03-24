fn main() {
   let student1:(&str,&str,&str) = ("Sylvia","ENG2901223","400L");
   //tuple that stores the name, mat number and level of the student
   let grade_student1:[i32;5] = [78,88,71,90,98];
   //arrays that stores the grade of the student in different courses.
   let course_code:[&str;5]=["CPE 331","CPE 366","CPE 322","CPE 455","CPE 333"];
   //the names of the courses the students did.this is for all students.

   //the same will be done for the remaining 10 students
   let student2:(&str,&str,&str) = ("Moses","ENG2208899","400L");
   let grade_student2:[i32;5] = [55,66,44,23,90];

   let student3:(&str,&str,&str) = ("Kylie","ENG2234455","400L");
   let grade_student3:[i32;5] = [78,44,68,80,99];

   //to calculate the sum of grades and average of each students
   //For the first student
   let sum1:i32 = grade_student1.iter().sum();
   let length1 = grade_student1.len() as f32;
   let average1 = sum1 as f32 / length1;
   println!("The average score of {} for {}, {}, {}, {} and {} is {}",student1.0,course_code[0],course_code[1],course_code[2],course_code[3],course_code[4],average1);

   //For the second student
   let sum2:i32 = grade_student2.iter().sum();
   let length2 = grade_student2.len() as f32;
   let average2 = sum2 as f32 / length2;
   println!("The average score of {} for {}, {}, {}, {} and {} is {}",student2.0,course_code[0],course_code[1],course_code[2],course_code[3],course_code[4],average2);

   //For the third student
   let sum3:i32 = grade_student3.iter().sum();
   let length3 = grade_student3.len() as f32;
   let average3 = sum3 as f32 / length3;
   println!("The average score of {} for {}, {}, {}, {} and {} is {}",student3.0,course_code[0],course_code[1],course_code[2],course_code[3],course_code[4],average3);

   //To group the highest scoring students. It creates a separate tuple for the comparism and stores it
   let (best_student, highest_avg) = if average1 > average2 && average1 > average3 {
      (student1, average1)
  } else if average2 > average1 && average2 > average3 {
      (student2, average2)
  } else {
      (student3, average3) 
  };
      println!("The student with the highest grade is {} with an average score of {:.2}",best_student.0, highest_avg);

  //Checks students that passed
  let mut i:usize=0;
  while i<length1 as usize{
   if grade_student1[i] < 50 {
      println!("{} FAILED",student1.0);
   }
   if grade_student2[i] < 50 {
      println!("{} FAILED",student2.0);
   }
   if grade_student3[i] < 50 {
      println!("{} FAILED",student3.0);
      i += 1;
   }
  }
}
