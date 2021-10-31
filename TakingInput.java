package com.Prajna;
import java.util.Scanner;

public class TakingInput {
    public static void main(String[] args) {
        System.out.println("Input from the user");
        Scanner input=new Scanner(System.in);

        System.out.println("Enter the first input");
//        int a =input.nextInt();
        float a= input.nextFloat();

        System.out.println("Enter the second input");
//        int b =input.nextInt();
        float b= input.nextFloat();

        System.out.println("The resultant of the two numbers is: ");
//        int sum =a+b;
        float sum=a+b;

        System.out.println(sum);

//        checks if the inout made by the user is integer or not
        boolean d= input.hasNextInt();
        System.out.println(d);

//        This inputs only one word string
        String str=input.next();
        System.out.println(str);


//        To input multiple words we use the below
        String strr=input.nextLine();
        System.out.println(strr);

    }
}
