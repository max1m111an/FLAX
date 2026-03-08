module io.flax {
    requires javafx.controls;
    requires javafx.fxml;

    requires org.controlsfx.controls;
    requires org.kordamp.bootstrapfx.core;

    opens io.flax to javafx.fxml;
    exports io.flax;
}