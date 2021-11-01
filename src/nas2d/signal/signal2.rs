


	/**
	 * Signal with preset number of parameters
	 *
	 * See https://github.com/lairworks/nas2d-core/wiki/Signal-&-Slots for usage documentation.
	 */
	template <typename... Params>
	class Signal : public SignalSource<Params...>
	{

		void emit(Params... params) const
		{
			for (auto& delegate : this->delegateList)
			{
				delegate(params...);
			}
		}

		void operator()(Params... params) const { emit(params...); }
	};

