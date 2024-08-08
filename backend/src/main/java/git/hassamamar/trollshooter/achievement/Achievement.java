package git.hassamamar.trollshooter.achievement;

import java.time.LocalDate;

public record Achievement(
        int id,
        String username,
        LocalDate unlockedAt
) {
    @Override
    public boolean equals(Object obj) {
        if (obj instanceof Achievement achievement) {
            return id == achievement.id && username.equals(achievement.username);
        } else {
            return false;
        }
    }
}
