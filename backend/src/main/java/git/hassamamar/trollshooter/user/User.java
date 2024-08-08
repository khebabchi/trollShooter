package git.hassamamar.trollshooter.user;

import java.time.LocalDate;

public record User(
        String username, // username is the id
        String email,
        String password,
        int topScore,
        LocalDate createdAt

) {
    @Override
    public boolean equals(Object obj) {
        if (obj instanceof User user) {
            return username.equals(user.username);
        }
        return false;
    }


    public UserInfo getInfo() {
        return new UserInfo(username, topScore);
    }

    public User emptyPassword() {
        return new User(username, email, "", topScore, createdAt);
    }

}
