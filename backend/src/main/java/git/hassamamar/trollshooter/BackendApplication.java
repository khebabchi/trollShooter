package git.hassamamar.trollshooter;

import git.hassamamar.trollshooter.achievement.AchievementRepository;
import git.hassamamar.trollshooter.user.UserRepository;
import org.springframework.boot.CommandLineRunner;
import org.springframework.boot.SpringApplication;
import org.springframework.boot.autoconfigure.SpringBootApplication;
import org.springframework.context.annotation.Bean;

@SpringBootApplication
public class BackendApplication {

    public static void main(String[] args) {
        SpringApplication.run(BackendApplication.class, args);

    }


    @Bean
    CommandLineRunner runner(AchievementRepository achievementRepository, UserRepository userRepository) {
        return args -> {
            System.out.println("AchievementRepository--------------------------------------------------------------");
            userRepository.insert("hocine", "amar@gmail.com", "password123");
            achievementRepository.insert(0, "amar");
            System.out.println("AchievementRepository--------------------------------------------------------------");
        };
    }
}
