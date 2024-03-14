import nlp from 'compromise';
import datePlugin from 'compromise-dates';
nlp.plugin(datePlugin);

export function extractDate(value: string) {
  // @ts-ignore
  let compromised = nlp(value).dates({"timezone": "Asia/Calcutta", "today": new Date().toISOString().slice(0,10)});
  let parsedDates = compromised.get()[0];
  console.log("parsed dates:", parsedDates);
  
  if (parsedDates && parsedDates.start && parsedDates.end) {
    let text = compromised.format('').all().text();
    // if the user puts the date in quotes, do not parse it
    let regexWithQuote = /\d{4}-\d{2}-\d{2} to \d{4}-\d{2}-\d{2}\"|\d{4}-\d{2}-\d{2}\"/g
    if (text.match(regexWithQuote) !== null && text.match(regexWithQuote).length > 0) {
      return null;
    } else {
      // find a date range (X to Y) or a single date (X)
      let regex = /\d{4}-\d{2}-\d{2} to \d{4}-\d{2}-\d{2}|\d{4}-\d{2}-\d{2}/g
      // remove dates from text
      text = text.replace(regex, '')

      let dateLimitTrimmed = {
        "start": parsedDates.start.slice(0,10),
        "end": parsedDates.end.slice(0,10),
        "text": text.trim().replace(/\s{2,}/g, ' ')
      }
      // make sure DD/MM is correctly parsed
      dateLimitTrimmed = checkDateMonth(dateLimitTrimmed, value);

      // convert to UNIX timestamp
      dateLimitTrimmed.start = new Date(dateLimitTrimmed.start + 'T00:00:00').getTime() / 1000;
      dateLimitTrimmed.end = new Date(dateLimitTrimmed.end + 'T23:59:59').getTime() / 1000;
      if (dateLimitTrimmed.start === dateLimitTrimmed.end) {
        dateLimitTrimmed.end = dateLimitTrimmed.start + 86400;
      }
      // save as string
      dateLimitTrimmed.start = dateLimitTrimmed.start.toString();
      dateLimitTrimmed.end = dateLimitTrimmed.end.toString();
      console.log(dateLimitTrimmed);
      return dateLimitTrimmed;
    }
  }
  return null;
}

function checkDateMonth(dateLimit: DateLimit, value: string) {
  console.log("received dates:", dateLimit);
  let defaultFormat = getDefaultDateFormat();
  console.log("Default Format:", defaultFormat);
  if (defaultFormat === 'MM/DD') {
    // we can trust compromise to do the right thing in this case
    return dateLimit;
  } else {
    // check if date(s) is explicitly present in the string
    let regex = /\d{2}-\d{2}|\d{2}\/\d{2}|\d{1}-\d{2}|\d{1}\/\d{2}|\d{2}-\d{1}|\d{2}\/\d{1}|\d{1}-\d{1}|\d{1}\/\d{1}/g;
    let matches = value.match(regex);
    if (matches) {
      console.log("Date Matches:", matches);
      console.log("Swapping day and month");
      // if the default format is DD/MM, the user probably wrote it that way
      // but compromise parsed it as MM/DD so swap the day and month
      let newStart = dateLimit.start.split('-').reverse().join('-');
      let newEnd = dateLimit.end.split('-').reverse().join('-');
      dateLimit.start = newStart;
      dateLimit.end = newEnd;
      console.log("Swapped dates:", dateLimit);
    }
    return dateLimit;
  }
}

// This may or may not work
// But I am keeping it here for now until further testing
function getDefaultDateFormat() {
  const dateString = new Intl.DateTimeFormat().format(new Date(2000, 1, 10)); // Parse 10th Feb 2000
  console.log(new Date().toLocaleString());
  console.log(dateString);
  const isMonthFirst = dateString.startsWith('2') || dateString.startsWith('02'); // check if if 10/2 or 2/10
  return isMonthFirst ? 'MM/DD' : 'DD/MM';
}

// Use '*' token to search even if word is not complete
// Examples – `hello world` will search for `hello* world*`
// `hello "world"` will search for `hello* "world"`
// `dear "star wars" fan` will search for `dear* "star wars" fan*`
export function cleanSearchQuery(value: string): string {
  // remove punctuation from the search query
  value = value.replace(/[[\]{}()*+?.,\\^$|#\s]/g, " ");
  // TODO: put double quotes on punctuation marks to make them work with the MATCH query
  
  // Split the input string into segments by space, but keep quoted strings together
  const segments = value.match(/"[^"]+"|\S+/g) || [];

  // Process each segment
  const processedSegments = segments.map(segment => {
    // If the segment is a quoted string, remove the quotes and don't add '*'
    if (segment.startsWith('"') && segment.endsWith('"')) {
      return segment.substring(1, segment.length - 1);
    }

    // If the segment has a - in front of it, replace the - with NOT and don't add '*'
    if (segment.startsWith('-')) {
      return `NOT ${segment.substring(1)}`;
    }

    // If the segment is a word, add '*'
    if (segment.match(/^[a-zA-Z0-9]+$/)) {
      return `${segment}*`;
    }

    // BUGFIX: Sometimes doublequote becomes “ or ”
    segment = segment.replace('“', '"').replace('”', '"')

    // Otherwise, return the segment as is
    return segment;
  });

  // Join the processed segments back together with spaces
  return processedSegments.join(' ');
}