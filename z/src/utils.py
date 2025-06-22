
#<·
def safe_exec(func):
    print(func)
    def wrapper(*args, **kwargs):
        try:
            result = func(*args, **kwargs)
            return result

        except Exception as e:
            print(e)
    return wrapper
