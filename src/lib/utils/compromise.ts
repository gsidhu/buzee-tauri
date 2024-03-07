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