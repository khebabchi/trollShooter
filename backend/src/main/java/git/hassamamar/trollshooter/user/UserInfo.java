package git.hassamamar.trollshooter.user;

public record UserInfo(
        String username, // username is the id
        int topScore

) {
    @Override
    public boolean equals(Object obj) {
        if (obj instanceof UserInfo user) {
            return username.equals(user.username);
        }
        return false;
    }
}
