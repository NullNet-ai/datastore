-- Check what data exists in organizations table
SELECT 
    status, 
    categories,
    COUNT(*) as count
FROM organizations 
WHERE tombstone = 0
GROUP BY status, categories
ORDER BY count DESC
LIMIT 20;