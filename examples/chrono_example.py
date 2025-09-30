from Rusthonian import chrono

def demo_datetime():
    print("=== DateTime Operations ===")
    now = chrono.DateTime.now()
    print(f"Current UTC time: {now}")
    print(f"Timestamp: {now.timestamp()}")
    print(f"Year: {now.year()}, Month: {now.month()}, Day: {now.day()}")
    print(f"Hour: {now.hour()}, Minute: {now.minute()}, Second: {now.second()}")
    print(f"RFC3339: {now.to_rfc3339()}")
    print(f"RFC2822: {now.to_rfc2822()}")
    print(f"Custom format: {now.format('%Y-%m-%d %H:%M:%S')}")
    print()

def demo_timezone():
    print("=== Timezone Operations ===")
    utc_time = chrono.now_utc()
    local_time = chrono.now_local()
    print(f"UTC: {utc_time}")
    print(f"Local: {local_time}")
    
    offset_plus_5 = chrono.FixedOffset.east(5 * 3600)
    print(f"Offset +5:00: {offset_plus_5}")
    
    dt = chrono.DateTime.now()
    fixed_dt = dt.to_fixed(offset_plus_5)
    print(f"DateTime with +5:00 offset: {fixed_dt}")
    print()

def demo_naive_types():
    print("=== Naive Date/Time (No Timezone) ===")
    date = chrono.NaiveDate(2024, 12, 25)
    print(f"Date: {date}")
    print(f"Weekday: {date.weekday()}")
    print(f"Day of year: {date.ordinal()}")
    
    time = chrono.NaiveTime(15, 30, 45)
    print(f"Time: {time}")
    print(f"Seconds from midnight: {time.num_seconds_from_midnight()}")
    
    datetime = date.and_time(time)
    print(f"Combined: {datetime}")
    
    datetime2 = chrono.NaiveDateTime(2024, 12, 25, 15, 30, 45)
    print(f"Direct creation: {datetime2}")
    print()

def demo_duration():
    print("=== Duration Operations ===")
    dur1 = chrono.Duration.hours(2)
    print(f"2 hours: {dur1}")
    print(f"In minutes: {dur1.num_minutes()}")
    print(f"In seconds: {dur1.num_seconds()}")
    
    dur2 = chrono.Duration(days=1, hours=2, minutes=30)
    print(f"1d 2h 30m: {dur2}")
    print(f"Total hours: {dur2.num_hours()}")
    
    dur3 = dur1 + dur2
    print(f"Sum: {dur3} ({dur3.num_hours()} hours)")
    
    now = chrono.DateTime.now()
    future = now + chrono.Duration.days(7)
    print(f"Now: {now}")
    print(f"7 days later: {future}")
    
    diff = future - now
    print(f"Difference: {diff.num_days()} days")
    print()

def demo_parsing():
    print("=== Parsing and Formatting ===")
    
    rfc3339_str = "2024-12-25T15:30:45Z"
    dt = chrono.parse_from_rfc3339(rfc3339_str)
    print(f"Parsed RFC3339 '{rfc3339_str}': {dt}")
    
    custom_str = "2024-12-25 15:30:45"
    dt2 = chrono.parse_datetime(custom_str, "%Y-%m-%d %H:%M:%S")
    print(f"Parsed custom '{custom_str}': {dt2}")
    
    date_str = "2024-12-25"
    date = chrono.parse_date(date_str, "%Y-%m-%d")
    print(f"Parsed date '{date_str}': {date}")
    
    time_str = "15:30:45"
    time = chrono.parse_time(time_str, "%H:%M:%S")
    print(f"Parsed time '{time_str}': {time}")
    print()

def demo_arithmetic():
    print("=== Date Arithmetic ===")
    date1 = chrono.NaiveDate(2024, 1, 1)
    date2 = chrono.NaiveDate(2024, 12, 31)
    
    diff = date2 - date1
    print(f"Days between {date1} and {date2}: {diff.num_days()}")
    
    new_date = date1 + chrono.Duration.days(100)
    print(f"{date1} + 100 days = {new_date}")
    
    dt1 = chrono.NaiveDateTime(2024, 1, 1, 12, 0, 0)
    dt2 = dt1 + chrono.Duration(hours=5, minutes=30)
    print(f"{dt1} + 5h30m = {dt2}")
    print()

def demo_timestamps():
    print("=== Timestamp Conversions ===")
    
    timestamp = 1703520645
    dt = chrono.DateTime.from_timestamp(timestamp, 0)
    print(f"From timestamp {timestamp}: {dt}")
    
    millis = 1703520645000
    dt2 = chrono.timestamp_millis(millis)
    print(f"From milliseconds {millis}: {dt2}")
    
    now = chrono.DateTime.now()
    print(f"Now: {now}")
    print(f"As timestamp: {now.timestamp()}")
    print(f"As millis: {now.timestamp_millis()}")
    print(f"As micros: {now.timestamp_micros()}")
    print()

def demo_weekday_month():
    print("=== Weekday and Month ===")
    
    monday = chrono.Weekday.monday()
    print(f"Monday: {monday}")
    print(f"Number from Monday: {monday.number_from_monday()}")
    print(f"Next day: {monday.succ()}")
    
    january = chrono.Month.january()
    print(f"January: {january}")
    print(f"Month number: {january.number_from_month()}")
    print(f"Next month: {january.succ()}")
    
    date = chrono.NaiveDate(2024, 12, 25)
    print(f"Date: {date}")
    print(f"Weekday: {date.weekday()}")
    print()

def demo_constants():
    print("=== Constants ===")
    print(f"MIN_YEAR: {chrono.MIN_YEAR}")
    print(f"MAX_YEAR: {chrono.MAX_YEAR}")
    print(f"MONDAY: {chrono.MONDAY}")
    print(f"JANUARY: {chrono.JANUARY}")
    print()

def demo_advanced():
    print("=== Advanced Operations ===")
    
    date = chrono.NaiveDate.from_yo(2024, 100)
    print(f"100th day of 2024: {date}")
    
    iso_date = chrono.NaiveDate.from_isoywd(2024, 52, 1)
    print(f"Week 52, Monday of 2024: {iso_date}")
    
    naive_dt = chrono.NaiveDateTime.from_timestamp(1703520645, 0)
    print(f"NaiveDateTime from timestamp: {naive_dt}")
    
    date = chrono.NaiveDate(2024, 3, 15)
    print(f"Date: {date}")
    print(f"ISO week: {date.iso_week()}")
    print(f"Days from CE: {date.num_days_from_ce()}")
    print()

def main():
    print("🦀 Rusthonian Chrono - Complete Demo\n")
    
    demo_datetime()
    demo_timezone()
    demo_naive_types()
    demo_duration()
    demo_parsing()
    demo_arithmetic()
    demo_timestamps()
    demo_weekday_month()
    demo_constants()
    demo_advanced()
    
    print("✅ All demos completed!")

if __name__ == "__main__":
    main()
