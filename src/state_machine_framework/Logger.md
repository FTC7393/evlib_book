Logger logs values in a text file with columns separated by tabs. This file can later be downloaded to your computer and read by a spreadsheet program to graph the values.

When you create it, you pass in:

* The file's name (e.g. `"log"`)
* The file's extension (e.g. `".csv"`)
* A list of `Logger.Column` objects, which contain
    * A column title to be put at the top of the file
    * An [InputExtractor](InputExtractor.md) of any type

Another column is automatically added at the beginning that logs the time in milliseconds

When the `start(File dir)` method is called, a file is created named `<filename><timestamp><file extension>` in the specified directory, for example `log1234567890.txt`. The timestamp is in milliseconds (System.currentTimeMillis()). Then the first line containing the column headers is written.

When the act() method is called, the InputExtractor for each Column is read, the results are connected with tabs, and the joined text is written to a new line in the file.

When the stop() method is called, the file is closed.

The getFileName() method returns the name of the log file.

[ftc/electronvolts/util/files/Logger.java](https://github.com/FTC7393/state-machine-framework/blob/master/src/ftc/electronvolts/util/files/Logger.java)
```java
package ftc.electronvolts.util.files;

import java.io.File;
import java.io.FileOutputStream;
import java.io.IOException;
import java.io.PrintStream;
import java.math.RoundingMode;
import java.text.DecimalFormat;
import java.util.List;

import ftc.electronvolts.util.InputExtractor;

/**
 * This file was made by the electronVolts, FTC team 7393
 *
 * A logger that takes a list of Columns, which have a header and an
 * InputExtractor, and logs each of them to a column in a file
 */
public class Logger {
    public static class Column {
        private final String header;
        private final InputExtractor<?> input;

        public Column(String header, InputExtractor<?> input) {
            this.header = header;
            this.input = input;
        }
    }

    private static final DecimalFormat df = new DecimalFormat("#.#####");
    static {
        df.setRoundingMode(RoundingMode.HALF_UP);
    }

    private long logStart;
    private PrintStream fileStream;
    private final String fileName, fileExtension;
    private String titles;
    private String fullFileName;
    private final List<Column> columns;

    /**
     * @param fileName the name of the file
     * @param fileExtension the file's extension (.txt for example)
     * @param columns the columns that will be written to the file
     */
    public Logger(String fileName, String fileExtension, List<Column> columns) {
        this.fileName = fileName;
        this.fileExtension = fileExtension;
        StringBuilder sb = new StringBuilder("time");
        for (Column column : columns) {
            sb.append("\t").append(column.header);
        }
        titles = sb.append("\n").toString();
        this.columns = columns;
    }

    /**
     * write the column titles to the file
     */
    public boolean start(File dir) {
        logStart = System.nanoTime();

        fullFileName = fileName + System.currentTimeMillis() + fileExtension;
        File file = new File(dir, fullFileName);
        try {
            fileStream = new PrintStream(new FileOutputStream(file));

            fileStream.printf(titles);
            return true;
        } catch (IOException e) {
            return false;
        }
    }

    /**
     * write the input columns to the file
     */
    public void act() {
        if (fileStream != null) {
            long now = System.nanoTime();
            StringBuilder line = new StringBuilder(df.format(1e-6 * (now - logStart)));
            for (Column column : columns) {
                line.append("\t").append(column.input.getValue());
            }
            fileStream.printf(line.append("\n").toString());
        }
    }

    /**
     * close the file
     */
    public void stop() {
        if (fileStream != null) {
            fileStream.close();
        }
    }

    public String getFileName() {
        return fullFileName;
    }
}
```