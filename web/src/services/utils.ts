export const sleep = (ms = 0) => {
  return new Promise((resolve) => setTimeout(resolve, ms))
}

/** Validation */
export const validators = {
  email: (v: string) => {
    const pattern = /^[^\s@]+@[^\s@]+\.[^\s@]+$/
    return pattern.test(v) || 'Please enter a valid email address'
  },
  required: (v: any) => !!v || 'This field is required',
}

export const parseDbDate = (date: String) => {
  var new_date = "";
  console.log(date);
  if (date) {
    var dates = date.split('T');
    var times = dates[1].split(':');
    dates = dates[0].split('-');
    new_date = dates[2] + "-" + dates[1] + '-' + dates[0] + " " + times[0] + ":" + times[1]+":"+times[2].split('.')[0];
  }
  return new_date;
}