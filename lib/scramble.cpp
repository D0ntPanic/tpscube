#include "scramble.h"


SimpleSeededRandomSource::SimpleSeededRandomSource(): m_seed(42)
{
}


SimpleSeededRandomSource::SimpleSeededRandomSource(uint32_t seed): m_seed(seed)
{
}


int SimpleSeededRandomSource::Next(int range)
{
	m_seed = (m_seed * 1103515245) + 12345;
	return m_seed % range;
}
